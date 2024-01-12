#pragma once
#define use_fastio
#ifdef use_fastio
#include "./fast_io/include/fast_io.h"
#endif

#include <bit>
#include <cassert>
#include <cmath>
#include <concepts>
#include <span>
#include <string>
#include <utility>

namespace na::nbt::v7 {
namespace impl {
enum class nbt_parse_error
{
    end_of_file,
    invalid
};

struct nbt_document;

namespace swapper {
template<typename T>
inline  T& byte_as_type(std::byte* ptr) noexcept;

template<typename T, std::endian nbt_endian>
requires(nbt_endian == std::endian::big || nbt_endian == std::endian::little) inline  T endian_get(std::byte* ptr) noexcept;

template<typename T, std::endian nbt_endian>
requires(nbt_endian == std::endian::big || nbt_endian == std::endian::little) inline  T endian_make_native_get(std::byte* ptr) noexcept;

template<typename T, std::endian nbt_endian>
requires(nbt_endian == std::endian::big || nbt_endian == std::endian::little) inline  T endian_get_make_native(std::byte* ptr) noexcept;

template<typename T, std::endian nbt_endian>
requires(nbt_endian == std::endian::big || nbt_endian == std::endian::little) inline  void endian_make_native(std::byte* ptr) noexcept;

template<typename read_swapper, std::endian nbt_endian>
concept read_swapper_for_endian = requires(std::byte * &t)
{
    read_swapper::template tag_byte<nbt_endian>(t);
    read_swapper::template tag_short<nbt_endian>(t);
    read_swapper::template tag_int<nbt_endian>(t);
    read_swapper::template tag_long<nbt_endian>(t);
    read_swapper::template tag_float<nbt_endian>(t);
    read_swapper::template tag_double<nbt_endian>(t);
    read_swapper::template tag_byte_array<nbt_endian>(t);
    read_swapper::template tag_string<nbt_endian>(t);
    read_swapper::template tag_int_array<nbt_endian>(t);
    read_swapper::template tag_long_array<nbt_endian>(t);
};
template<typename read_swapper_t>
concept read_swapper =
    read_swapper_for_endian<read_swapper_t, std::endian::big> &&
    read_swapper_for_endian<read_swapper_t, std::endian::little>;

template<typename write_swapper, std::endian nbt_endian>
concept write_swapper_for_endian = requires(std::byte * &t)
{
    write_swapper::template tag_byte<nbt_endian>(t);
    write_swapper::template tag_short<nbt_endian>(t);
    write_swapper::template tag_int<nbt_endian>(t);
    write_swapper::template tag_long<nbt_endian>(t);
    write_swapper::template tag_float<nbt_endian>(t);
    write_swapper::template tag_double<nbt_endian>(t);
    write_swapper::template tag_byte_array<nbt_endian>(t);
    write_swapper::template tag_string<nbt_endian>(t);
    write_swapper::template tag_int_array<nbt_endian>(t);
    write_swapper::template tag_long_array<nbt_endian>(t);
};
template<typename write_swapper_t>
concept write_swapper =
    write_swapper_for_endian<write_swapper_t, std::endian::big> &&
    write_swapper_for_endian<write_swapper_t, std::endian::little>;

}  // namespace swapper
namespace read_write {
template<bool in_place, bool bound_check, std::endian nbt_endian, swapper::read_swapper rswap>
[[nodiscard]] inline  auto read(std::byte* source, std::size_t source_len) -> nbt_document;

}  // namespace read_write
union mark_t
{
    struct
    {
        uint32_t general_parrent_offset;  // 4
        uint32_t list_current_length;     // 4
        uint32_t list_total_length;       // 4
        uint16_t list_type;               // 2
        uint16_t general_is_compound;     // 2
    } cache;
    struct
    {
        uint64_t flat_next_mark;  // offset from this
        std::byte* end;
    } store;
};

struct nbt_document
{
    mark_t* mark_m;
    std::byte* source_m;

    mark_t* mark;
    std::byte* source;
    std::size_t mark_len;    //可能小于 mark_m 已分配内存
    std::size_t source_len;  //可能小于 source_m 已分配内存
};

enum class nbt_type : ::std::uint8_t
{
    tag_end = 0,
    tag_byte = 1,
    tag_short = 2,
    tag_int = 3,
    tag_long = 4,
    tag_float = 5,
    tag_double = 6,
    tag_byte_array = 7,
    tag_string = 8,
    tag_list = 9,
    tag_compound = 10,
    tag_int_array = 11,
    tag_long_array = 12
};

struct any_tag
{
    mark_t* mark;
    std::byte* source;

#ifndef NDEBUG
    nbt_type type;
#endif
};

struct nbt_list
{
    struct iterator
    {
        mark_t* mark;
        std::byte* source;

        std::int32_t index;

#ifndef NDEBUG
        nbt_type type;
#endif
    };
#ifndef NDEBUG
    nbt_type element_type;
#endif
    std::int32_t length;
    mark_t* mark;
    std::byte* source;
};

struct nbt_compound
{
    struct iterator
    {
        mark_t* mark;
        std::byte* source;
    };
    mark_t* mark;
    std::byte* source;
};

namespace nbt_document_function {

/// <summary>
/// 释放 doc 所有的内存并置空所有指针.
/// </summary>
/// <param name="doc"></param>
inline  void nbt_document_free(nbt_document* doc) noexcept
{
    doc->source = nullptr;
    doc->mark = nullptr;
    if (doc->source_m != nullptr)
    {
        #ifdef use_fastio
        fast_io::native_global_allocator::deallocate(doc->source_m);
        #else
        free(doc->source_m);
        #endif
    }
    if (doc->mark_m != nullptr)
    {
        #ifdef use_fastio
        fast_io::native_global_allocator::deallocate(doc->mark_m);
        #else
        free(doc->mark_m);
        #endif
    }
}

/// <summary>
/// 将 from 中已分配内存的所有权转移到 to.
/// to 中不能所有已分配内存，否则会导致泄漏.
/// 移动之后，from 中所有指针被置空.
/// </summary>
/// <param name="from"></param>
/// <param name="to"></param>
inline  void nbt_document_move(nbt_document* from, nbt_document* to) noexcept
{
    *to = *from;
    from->mark_m = nullptr;
    from->source_m = nullptr;
    from->mark = nullptr;
    from->source = nullptr;
}

inline  const std::u8string_view nbt_document_root_key(const nbt_document* doc) noexcept
{
    if (static_cast<nbt_type>(swapper::byte_as_type<std::uint8_t>(doc->source)) == nbt_type::tag_end) [[unlikely]]
    {
        return {};
    }
    return std::u8string_view(
        reinterpret_cast<char8_t*>(doc->source + sizeof(std::uint8_t)),
        swapper::byte_as_type<std::uint16_t>(doc->source + sizeof(std::uint8_t)));
}

inline  any_tag nbt_document_root_value(const nbt_document* doc) noexcept
{
    return any_tag{
        .mark = doc->mark,
        .source = doc->source + sizeof(std::uint8_t) + sizeof(std::uint16_t) + swapper::byte_as_type<std::uint16_t>(doc->source + sizeof(std::uint8_t))
#ifndef NDEBUG
            ,
        .type = static_cast<nbt_type>(swapper::byte_as_type<std::uint8_t>(doc->source))
#endif
    };
}
}  // namespace nbt_document_function
namespace any_tag_function {

inline  auto any_tag_get_end(const any_tag* tag) noexcept
{
    assert(tag->type == nbt_type::tag_end);
    return;
}

inline  auto any_tag_get_byte(const any_tag* tag) noexcept
{
    assert(tag->type == nbt_type::tag_byte);
    return swapper::byte_as_type<std::int8_t>(tag->source);
}

inline  auto any_tag_get_short(const any_tag* tag) noexcept
{
    assert(tag->type == nbt_type::tag_short);
    return swapper::byte_as_type<std::int16_t>(tag->source);
}

inline  auto any_tag_get_int(const any_tag* tag) noexcept
{
    assert(tag->type == nbt_type::tag_int);
    return swapper::byte_as_type<std::int32_t>(tag->source);
}

inline  auto any_tag_get_long(const any_tag* tag) noexcept
{
    assert(tag->type == nbt_type::tag_long);
    return swapper::byte_as_type<std::int64_t>(tag->source);
}

inline  auto any_tag_get_float(const any_tag* tag) noexcept
{
    assert(tag->type == nbt_type::tag_float);
    return swapper::byte_as_type<std::float_t>(tag->source);
}

inline  auto any_tag_get_double(const any_tag* tag) noexcept
{
    assert(tag->type == nbt_type::tag_double);
    return swapper::byte_as_type<std::double_t>(tag->source);
}

inline  auto any_tag_get_byte_array(const any_tag* tag) noexcept
{
    assert(tag->type == nbt_type::tag_byte_array);
    return std::span<const std::int8_t, std::dynamic_extent>(reinterpret_cast<std::int8_t*>(tag->source + sizeof(std::int32_t)), swapper::byte_as_type<std::int32_t>(tag->source));
}

inline  auto any_tag_get_string(const any_tag* tag) noexcept
{
    assert(tag->type == nbt_type::tag_string);
    return std::u8string_view(reinterpret_cast<char8_t*>(tag->source + sizeof(std::uint16_t)), swapper::byte_as_type<std::uint16_t>(tag->source));
}

inline  auto any_tag_get_list_end(const any_tag* tag) noexcept
{
    assert(tag->type == nbt_type::tag_list);
    assert(static_cast<nbt_type>(swapper::byte_as_type<std::uint8_t>(tag->source)) == nbt_type::tag_end);
    return;
}

inline  auto any_tag_get_list_byte(const any_tag* tag) noexcept
{
    assert(tag->type == nbt_type::tag_list);
    assert(static_cast<nbt_type>(swapper::byte_as_type<std::uint8_t>(tag->source)) == nbt_type::tag_byte);
    return std::span<std::int8_t, std::dynamic_extent>(
        reinterpret_cast<std::int8_t*>(tag->source + sizeof(std::uint8_t) + sizeof(std::int32_t)),
        swapper::byte_as_type<std::int32_t>(tag->source + sizeof(std::uint8_t)));
}

inline  auto any_tag_get_list_short(const any_tag* tag) noexcept
{
    assert(tag->type == nbt_type::tag_list);
    assert(static_cast<nbt_type>(swapper::byte_as_type<std::uint8_t>(tag->source)) == nbt_type::tag_short);
    return std::span<std::int16_t, std::dynamic_extent>(
        reinterpret_cast<std::int16_t*>(tag->source + sizeof(std::uint8_t) + sizeof(std::int32_t)),
        swapper::byte_as_type<std::int32_t>(tag->source + sizeof(std::uint8_t)));
}

inline  auto any_tag_get_list_int(const any_tag* tag) noexcept
{
    assert(tag->type == nbt_type::tag_list);
    assert(static_cast<nbt_type>(swapper::byte_as_type<std::uint8_t>(tag->source)) == nbt_type::tag_int);
    return std::span<std::int32_t, std::dynamic_extent>(
        reinterpret_cast<std::int32_t*>(tag->source + sizeof(std::uint8_t) + sizeof(std::int32_t)),
        swapper::byte_as_type<std::int32_t>(tag->source + sizeof(std::uint8_t)));
}

inline  auto any_tag_get_list_long(const any_tag* tag) noexcept
{
    assert(tag->type == nbt_type::tag_list);
    assert(static_cast<nbt_type>(swapper::byte_as_type<std::uint8_t>(tag->source)) == nbt_type::tag_long);
    return std::span<std::int64_t, std::dynamic_extent>(
        reinterpret_cast<std::int64_t*>(tag->source + sizeof(std::uint8_t) + sizeof(std::int32_t)),
        swapper::byte_as_type<std::int32_t>(tag->source + sizeof(std::uint8_t)));
}

inline  auto any_tag_get_list_float(const any_tag* tag) noexcept
{
    assert(tag->type == nbt_type::tag_list);
    assert(static_cast<nbt_type>(swapper::byte_as_type<std::uint8_t>(tag->source)) == nbt_type::tag_float);
    return std::span<std::float_t, std::dynamic_extent>(
        reinterpret_cast<std::float_t*>(tag->source + sizeof(std::uint8_t) + sizeof(std::int32_t)),
        swapper::byte_as_type<std::int32_t>(tag->source + sizeof(std::uint8_t)));
}

inline  auto any_tag_get_list_double(const any_tag* tag) noexcept
{
    assert(tag->type == nbt_type::tag_list);
    assert(static_cast<nbt_type>(swapper::byte_as_type<std::uint8_t>(tag->source)) == nbt_type::tag_double);
    return std::span<std::double_t, std::dynamic_extent>(
        reinterpret_cast<std::double_t*>(tag->source + sizeof(std::uint8_t) + sizeof(std::int32_t)),
        swapper::byte_as_type<std::int32_t>(tag->source + sizeof(std::uint8_t)));
}

inline  auto any_tag_get_list_byte_array(const any_tag* tag) noexcept
{
    assert(tag->type == nbt_type::tag_list);
    assert(static_cast<nbt_type>(swapper::byte_as_type<std::uint8_t>(tag->source)) == nbt_type::tag_byte_array);
    return nbt_list{
#ifndef NDEBUG
        .element_type = nbt_type::tag_byte_array,
#endif
        .length = swapper::byte_as_type<std::int32_t>(tag->source + sizeof(std::uint8_t)),
        .mark = tag->mark,
        .source = tag->source + sizeof(std::uint8_t) + sizeof(std::int32_t)};
}

inline  auto any_tag_get_list_string(const any_tag* tag) noexcept
{
    assert(tag->type == nbt_type::tag_list);
    assert(static_cast<nbt_type>(swapper::byte_as_type<std::uint8_t>(tag->source)) == nbt_type::tag_string);
    return nbt_list{
#ifndef NDEBUG
        .element_type = nbt_type::tag_string,
#endif
        .length = swapper::byte_as_type<std::int32_t>(tag->source + sizeof(std::uint8_t)),
        .mark = tag->mark,
        .source = tag->source + sizeof(std::uint8_t) + sizeof(std::int32_t)};
}

inline  auto any_tag_get_list_list(const any_tag* tag) noexcept
{
    assert(tag->type == nbt_type::tag_list);
    assert(static_cast<nbt_type>(swapper::byte_as_type<std::uint8_t>(tag->source)) == nbt_type::tag_list);
    return nbt_list{
#ifndef NDEBUG
        .element_type = nbt_type::tag_list,
#endif
        .length = swapper::byte_as_type<std::int32_t>(tag->source + sizeof(std::uint8_t)),
        .mark = tag->mark,
        .source = tag->source + sizeof(std::uint8_t) + sizeof(std::int32_t)};
}

inline  auto any_tag_get_list_compound(const any_tag* tag) noexcept
{
    assert(tag->type == nbt_type::tag_list);
    assert(static_cast<nbt_type>(swapper::byte_as_type<std::uint8_t>(tag->source)) == nbt_type::tag_compound);
    return nbt_list{
#ifndef NDEBUG
        .element_type = nbt_type::tag_compound,
#endif
        .length = swapper::byte_as_type<std::int32_t>(tag->source + sizeof(std::uint8_t)),
        .mark = tag->mark,
        .source = tag->source + sizeof(std::uint8_t) + sizeof(std::int32_t)};
}

inline  auto any_tag_get_list_int_array(const any_tag* tag) noexcept
{
    assert(tag->type == nbt_type::tag_list);
    assert(static_cast<nbt_type>(swapper::byte_as_type<std::uint8_t>(tag->source)) == nbt_type::tag_int_array);
    return nbt_list{
#ifndef NDEBUG
        .element_type = nbt_type::tag_int_array,
#endif
        .length = swapper::byte_as_type<std::int32_t>(tag->source + sizeof(std::uint8_t)),
        .mark = tag->mark,
        .source = tag->source + sizeof(std::uint8_t) + sizeof(std::int32_t)};
}

inline  auto any_tag_get_list_long_array(const any_tag* tag) noexcept
{
    assert(tag->type == nbt_type::tag_list);
    assert(static_cast<nbt_type>(swapper::byte_as_type<std::uint8_t>(tag->source)) == nbt_type::tag_long_array);
    return nbt_list{
#ifndef NDEBUG
        .element_type = nbt_type::tag_long_array,
#endif
        .length = swapper::byte_as_type<std::int32_t>(tag->source + sizeof(std::uint8_t)),
        .mark = tag->mark,
        .source = tag->source + sizeof(std::uint8_t) + sizeof(std::int32_t)};
}

inline  auto any_tag_get_compound(const any_tag* tag) noexcept
{
    assert(tag->type == nbt_type::tag_compound);
    return nbt_compound{
        .mark = tag->mark,
        .source = tag->source};
}

inline  auto any_tag_get_int_array(const any_tag* tag) noexcept
{
    assert(tag->type == nbt_type::tag_int_array);
    return std::span<const std::int32_t, std::dynamic_extent>(reinterpret_cast<std::int32_t*>(tag->source + sizeof(std::int32_t)), swapper::byte_as_type<std::int32_t>(tag->source));
}

inline  auto any_tag_get_long_array(const any_tag* tag) noexcept
{
    assert(tag->type == nbt_type::tag_long_array);
    return std::span<const std::int64_t, std::dynamic_extent>(reinterpret_cast<std::int64_t*>(tag->source + sizeof(std::int32_t)), swapper::byte_as_type<std::int32_t>(tag->source));
}

inline  auto any_tag_valid(const any_tag* tag) noexcept
{
    return tag->mark != nullptr && tag->source != nullptr
#ifndef NDEBUG
           && tag->type == nbt_type::tag_end
#endif
        ;
}

}  // namespace any_tag_function
namespace nbt_compound_function {
inline  auto nbt_compound_iterator_begin(const nbt_compound* comp) noexcept
{
    return nbt_compound::iterator{
        .mark = comp->mark + 1,
        .source = comp->source};
}
inline  auto nbt_compound_iterator_end(const nbt_compound* comp) noexcept
{
    return nbt_compound::iterator{
        .mark = nullptr,
        .source = comp->mark->store.end - 1  //at tag_end
    };
}
inline  void nbt_compound_iterator_next(nbt_compound::iterator* iter) noexcept
{
    assert(iter->mark != nullptr);
    assert(iter->source != nullptr);
    auto type = static_cast<nbt_type>(swapper::byte_as_type<std::uint8_t>(iter->source));
    assert(type != nbt_type::tag_end);
    auto len = swapper::byte_as_type<::std::uint16_t>(iter->source + 1);  //name length

    switch (type)
    {
        case nbt_type::tag_byte:
            iter->source = iter->source + 1 + 2 + len + 1;
            return;
        case nbt_type::tag_short:
            iter->source = iter->source + 1 + 2 + len + 2;
            return;
        case nbt_type::tag_int:
            iter->source = iter->source + 1 + 2 + len + 4;
            return;
        case nbt_type::tag_long:
            iter->source = iter->source + 1 + 2 + len + 8;
            return;
        case nbt_type::tag_float:
            iter->source = iter->source + 1 + 2 + len + 4;
            return;
        case nbt_type::tag_double:
            iter->source = iter->source + 1 + 2 + len + 8;
            return;
        case nbt_type::tag_string:
            iter->source = iter->source + 1 + 2 + len;  //payload begin
            iter->source = iter->source + 2 + swapper::byte_as_type<::std::uint16_t>(iter->source);
            return;
        case nbt_type::tag_byte_array:
            iter->source = iter->source + 1 + 2 + len;  //payload begin
            iter->source = iter->source + 4 + swapper::byte_as_type<::std::int32_t>(iter->source);
            return;
        case nbt_type::tag_int_array:
            iter->source = iter->source + 1 + 2 + len;  //payload begin
            iter->source = iter->source + 4 + static_cast<std::ptrdiff_t>(swapper::byte_as_type<::std::int32_t>(iter->source)) * 4;
            return;
        case nbt_type::tag_long_array:
            iter->source = iter->source + 1 + 2 + len;  //payload begin
            iter->source = iter->source + 4 + static_cast<std::ptrdiff_t>(swapper::byte_as_type<::std::int32_t>(iter->source)) * 8;
            return;
        case nbt_type::tag_list:
        case nbt_type::tag_compound:
            iter->source = iter->mark->store.end;
            iter->mark = iter->mark + iter->mark->store.flat_next_mark;
            return;
        default:
            std::unreachable();
    }
}

inline  bool nbt_compound_iter_equal(const nbt_compound::iterator* left, const nbt_compound::iterator* right)
{
    assert(left->source != nullptr);
    assert(right->source != nullptr);
    return left->source == right->source;
}
inline  const std::u8string_view nbt_compound_iter_key(const nbt_compound::iterator* iter) noexcept
{
    assert(iter->mark != nullptr);
    assert(iter->source != nullptr);
    assert(static_cast<nbt_type>(swapper::byte_as_type<std::uint8_t>(iter->source)) != nbt_type::tag_end);
    return std::u8string_view(reinterpret_cast<char8_t*>(iter->source + sizeof(std::uint8_t) + sizeof(std::uint16_t)), swapper::byte_as_type<std::uint16_t>(iter->source + sizeof(std::uint8_t)));
}
inline  any_tag nbt_compound_iter_value(const nbt_compound::iterator* iter) noexcept
{
    assert(iter->mark != nullptr);
    assert(iter->source != nullptr);
    assert(static_cast<nbt_type>(swapper::byte_as_type<std::uint8_t>(iter->source)) != nbt_type::tag_end);
    return any_tag{
        .mark = iter->mark,
        .source = iter->source + sizeof(std::uint8_t) + sizeof(std::uint16_t) + swapper::byte_as_type<std::uint16_t>(iter->source + sizeof(std::uint8_t))
#ifndef NDEBUG
            ,
        .type = static_cast<nbt_type>(swapper::byte_as_type<std::uint8_t>(iter->source))
#endif
    };
}

inline  auto nbt_compound_find_value(const std::u8string_view key, const nbt_compound::iterator* begin, const nbt_compound::iterator* end) noexcept
{
    for (auto iter{*begin}; !nbt_compound_iter_equal(std::addressof(iter), end); nbt_compound_iterator_next(std::addressof(iter)))
    {
        if (nbt_compound_iter_key(std::addressof(iter)).compare(key) == 0)
        {
            return iter;
        }
    }
    return *end;
}

inline  auto nbt_compound_find_value(const nbt_compound* comp, const std::u8string_view key) noexcept
{
    auto begin{nbt_compound_iterator_begin(comp)};
    auto end{nbt_compound_iterator_end(comp)};
    return nbt_compound_find_value(key, std::addressof(begin), std::addressof(end));
}
}  // namespace nbt_compound_function
namespace nbt_list_function {
inline  auto nbt_list_iterator_begin(const nbt_list* list) noexcept
{
    return nbt_list::iterator{
        .mark = list->mark + 1,
        .source = list->source,
        .index = 0
#ifndef NDEBUG
        ,
        .type = list->element_type
#endif
    };
}
inline  auto nbt_list_iterator_end(const nbt_list* list) noexcept
{
    return nbt_list::iterator{
        .mark = nullptr,
        .source = list->mark->store.end,
        .index = list->length};
}

inline  void nbt_list_iterator_next_end(nbt_list::iterator* iter) noexcept
{
    assert(iter->mark != nullptr);
    assert(iter->source != nullptr);
    assert(iter->type == nbt_type::tag_end);

    iter->index++;
    return;
}

inline  void nbt_list_iterator_next_byte(nbt_list::iterator* iter) noexcept
{
    assert(iter->mark != nullptr);
    assert(iter->source != nullptr);
    assert(iter->type == nbt_type::tag_byte);

    iter->index++;
    iter->source = iter->source + 1;
    return;
}

inline  void nbt_list_iterator_next_short(nbt_list::iterator* iter) noexcept
{
    assert(iter->mark != nullptr);
    assert(iter->source != nullptr);
    assert(iter->type == nbt_type::tag_short);

    iter->index++;
    iter->source = iter->source + 2;
    return;
}

inline  void nbt_list_iterator_next_int(nbt_list::iterator* iter) noexcept
{
    assert(iter->mark != nullptr);
    assert(iter->source != nullptr);
    assert(iter->type == nbt_type::tag_int);

    iter->index++;
    iter->source = iter->source + 4;
    return;
}

inline  void nbt_list_iterator_next_long(nbt_list::iterator* iter) noexcept
{
    assert(iter->mark != nullptr);
    assert(iter->source != nullptr);
    assert(iter->type == nbt_type::tag_long);

    iter->index++;
    iter->source = iter->source + 8;
    return;
}

inline  void nbt_list_iterator_next_float(nbt_list::iterator* iter) noexcept
{
    assert(iter->mark != nullptr);
    assert(iter->source != nullptr);
    assert(iter->type == nbt_type::tag_float);

    iter->index++;
    iter->source = iter->source + 4;
    return;
}

inline  void nbt_list_iterator_next_double(nbt_list::iterator* iter) noexcept
{
    assert(iter->mark != nullptr);
    assert(iter->source != nullptr);
    assert(iter->type == nbt_type::tag_double);

    iter->index++;
    iter->source = iter->source + 8;
    return;
}

inline  void nbt_list_iterator_next_string(nbt_list::iterator* iter) noexcept
{
    assert(iter->mark != nullptr);
    assert(iter->source != nullptr);
    assert(iter->type == nbt_type::tag_string);

    iter->index++;
    iter->source = iter->source + 2 + swapper::byte_as_type<::std::uint16_t>(iter->source);
    return;
}

inline  void nbt_list_iterator_next_byte_array(nbt_list::iterator* iter) noexcept
{
    assert(iter->mark != nullptr);
    assert(iter->source != nullptr);
    assert(iter->type == nbt_type::tag_byte_array);

    iter->index++;
    iter->source = iter->source + 4 + swapper::byte_as_type<::std::int32_t>(iter->source);
    return;
}

inline  void nbt_list_iterator_next_int_array(nbt_list::iterator* iter) noexcept
{
    assert(iter->mark != nullptr);
    assert(iter->source != nullptr);
    assert(iter->type == nbt_type::tag_int_array);

    iter->index++;
    iter->source = iter->source + 4 + swapper::byte_as_type<::std::int32_t>(iter->source) * 4;
    return;
}

inline  void nbt_list_iterator_next_long_array(nbt_list::iterator* iter) noexcept
{
    assert(iter->mark != nullptr);
    assert(iter->source != nullptr);
    assert(iter->type == nbt_type::tag_long_array);

    iter->index++;
    iter->source = iter->source + 4 + swapper::byte_as_type<::std::int32_t>(iter->source) * 8;
    return;
}

inline  void nbt_list_iterator_next_list(nbt_list::iterator* iter) noexcept
{
    assert(iter->mark != nullptr);
    assert(iter->source != nullptr);
    assert(iter->type == nbt_type::tag_list);

    iter->index++;
    iter->source = iter->mark->store.end;
    iter->mark = iter->mark + iter->mark->store.flat_next_mark;
    return;
}

inline  void nbt_list_iterator_next_compound(nbt_list::iterator* iter) noexcept
{
    assert(iter->mark != nullptr);
    assert(iter->source != nullptr);
    assert(iter->type == nbt_type::tag_compound);

    iter->index++;
    iter->source = iter->mark->store.end;
    iter->mark = iter->mark + iter->mark->store.flat_next_mark;
    return;
}

inline  bool nbt_list_iter_equal(const nbt_list::iterator* left, const nbt_list::iterator* right)
{
    assert(left->source != nullptr);
    assert(right->source != nullptr);
    return left->source == right->source;
}
inline  any_tag nbt_list_iter_value(const nbt_list::iterator* iter) noexcept
{
    assert(iter->mark != nullptr);
    assert(iter->source != nullptr);
    return any_tag{
        .mark = iter->mark,
        .source = iter->source
#ifndef NDEBUG
        ,
        .type = iter->type
#endif
    };
}

inline  auto nbt_list_find_value_end(std::int32_t index, const nbt_list::iterator* begin, const nbt_list::iterator* end) noexcept
{
    auto iter{*begin};
    while (index--)
    {
        nbt_list_iterator_next_end(std::addressof(iter));
    }
    return iter;
}

inline  auto nbt_list_find_value_byte(std::int32_t index, const nbt_list::iterator* begin, const nbt_list::iterator* end) noexcept
{
    auto iter{*begin};
    while (index--)
    {
        nbt_list_iterator_next_byte(std::addressof(iter));
    }
    return iter;
}

inline  auto nbt_list_find_value_short(std::int32_t index, const nbt_list::iterator* begin, const nbt_list::iterator* end) noexcept
{
    auto iter{*begin};
    while (index--)
    {
        nbt_list_iterator_next_short(std::addressof(iter));
    }
    return iter;
}

inline  auto nbt_list_find_value_int(std::int32_t index, const nbt_list::iterator* begin, const nbt_list::iterator* end) noexcept
{
    auto iter{*begin};
    while (index--)
    {
        nbt_list_iterator_next_int(std::addressof(iter));
    }
    return iter;
}

inline  auto nbt_list_find_value_long(std::int32_t index, const nbt_list::iterator* begin, const nbt_list::iterator* end) noexcept
{
    auto iter{*begin};
    while (index--)
    {
        nbt_list_iterator_next_long(std::addressof(iter));
    }
    return iter;
}

inline  auto nbt_list_find_value_float(std::int32_t index, const nbt_list::iterator* begin, const nbt_list::iterator* end) noexcept
{
    auto iter{*begin};
    while (index--)
    {
        nbt_list_iterator_next_float(std::addressof(iter));
    }
    return iter;
}

inline  auto nbt_list_find_value_double(std::int32_t index, const nbt_list::iterator* begin, const nbt_list::iterator* end) noexcept
{
    auto iter{*begin};
    while (index--)
    {
        nbt_list_iterator_next_double(std::addressof(iter));
    }
    return iter;
}

inline  auto nbt_list_find_value_byte_array(std::int32_t index, const nbt_list::iterator* begin, const nbt_list::iterator* end) noexcept
{
    auto iter{*begin};
    while (index--)
    {
        nbt_list_iterator_next_byte_array(std::addressof(iter));
    }
    return iter;
}

inline  auto nbt_list_find_value_string(std::int32_t index, const nbt_list::iterator* begin, const nbt_list::iterator* end) noexcept
{
    auto iter{*begin};
    while (index--)
    {
        nbt_list_iterator_next_string(std::addressof(iter));
    }
    return iter;
}

inline  auto nbt_list_find_value_list(std::int32_t index, const nbt_list::iterator* begin, const nbt_list::iterator* end) noexcept
{
    auto iter{*begin};
    while (index--)
    {
        nbt_list_iterator_next_list(std::addressof(iter));
    }
    return iter;
}

inline  auto nbt_list_find_value_compound(std::int32_t index, const nbt_list::iterator* begin, const nbt_list::iterator* end) noexcept
{
    auto iter{*begin};
    while (index--)
    {
        nbt_list_iterator_next_compound(std::addressof(iter));
    }
    return iter;
}

inline  auto nbt_list_find_value_int_array(std::int32_t index, const nbt_list::iterator* begin, const nbt_list::iterator* end) noexcept
{
    auto iter{*begin};
    while (index--)
    {
        nbt_list_iterator_next_int_array(std::addressof(iter));
    }
    return iter;
}

inline  auto nbt_list_find_value_long_array(std::int32_t index, const nbt_list::iterator* begin, const nbt_list::iterator* end) noexcept
{
    auto iter{*begin};
    while (index--)
    {
        nbt_list_iterator_next_long_array(std::addressof(iter));
    }
    return iter;
}

inline  auto nbt_list_find_value_end(const nbt_list* list, std::int32_t index) noexcept
{
    assert(list->mark != nullptr);
    assert(list->source != nullptr);
    assert(list->element_type == nbt_type::tag_end);
    auto begin{nbt_list_iterator_begin(list)};
    auto end{nbt_list_iterator_end(list)};
    if (index >= list->length)
    {
        return end;
    }
    return nbt_list_find_value_end(index, std::addressof(begin), std::addressof(end));
}

inline  auto nbt_list_find_value_byte(const nbt_list* list, std::int32_t index) noexcept
{
    assert(list->mark != nullptr);
    assert(list->source != nullptr);
    assert(list->element_type == nbt_type::tag_byte);
    auto begin{nbt_list_iterator_begin(list)};
    auto end{nbt_list_iterator_end(list)};
    if (index >= list->length)
    {
        return end;
    }
    return nbt_list_find_value_byte(index, std::addressof(begin), std::addressof(end));
}

inline  auto nbt_list_find_value_short(const nbt_list* list, std::int32_t index) noexcept
{
    assert(list->mark != nullptr);
    assert(list->source != nullptr);
    assert(list->element_type == nbt_type::tag_short);
    auto begin{nbt_list_iterator_begin(list)};
    auto end{nbt_list_iterator_end(list)};
    if (index >= list->length)
    {
        return end;
    }
    return nbt_list_find_value_short(index, std::addressof(begin), std::addressof(end));
}

inline  auto nbt_list_find_value_int(const nbt_list* list, std::int32_t index) noexcept
{
    assert(list->mark != nullptr);
    assert(list->source != nullptr);
    assert(list->element_type == nbt_type::tag_int);
    auto begin{nbt_list_iterator_begin(list)};
    auto end{nbt_list_iterator_end(list)};
    if (index >= list->length)
    {
        return end;
    }
    return nbt_list_find_value_int(index, std::addressof(begin), std::addressof(end));
}

inline  auto nbt_list_find_value_long(const nbt_list* list, std::int32_t index) noexcept
{
    assert(list->mark != nullptr);
    assert(list->source != nullptr);
    assert(list->element_type == nbt_type::tag_long);
    auto begin{nbt_list_iterator_begin(list)};
    auto end{nbt_list_iterator_end(list)};
    if (index >= list->length)
    {
        return end;
    }
    return nbt_list_find_value_long(index, std::addressof(begin), std::addressof(end));
}

inline  auto nbt_list_find_value_float(const nbt_list* list, std::int32_t index) noexcept
{
    assert(list->mark != nullptr);
    assert(list->source != nullptr);
    assert(list->element_type == nbt_type::tag_float);
    auto begin{nbt_list_iterator_begin(list)};
    auto end{nbt_list_iterator_end(list)};
    if (index >= list->length)
    {
        return end;
    }
    return nbt_list_find_value_float(index, std::addressof(begin), std::addressof(end));
}

inline  auto nbt_list_find_value_double(const nbt_list* list, std::int32_t index) noexcept
{
    assert(list->mark != nullptr);
    assert(list->source != nullptr);
    assert(list->element_type == nbt_type::tag_double);
    auto begin{nbt_list_iterator_begin(list)};
    auto end{nbt_list_iterator_end(list)};
    if (index >= list->length)
    {
        return end;
    }
    return nbt_list_find_value_double(index, std::addressof(begin), std::addressof(end));
}

inline  auto nbt_list_find_value_byte_array(const nbt_list* list, std::int32_t index) noexcept
{
    assert(list->mark != nullptr);
    assert(list->source != nullptr);
    assert(list->element_type == nbt_type::tag_byte_array);
    auto begin{nbt_list_iterator_begin(list)};
    auto end{nbt_list_iterator_end(list)};
    if (index >= list->length)
    {
        return end;
    }
    return nbt_list_find_value_byte_array(index, std::addressof(begin), std::addressof(end));
}

inline  auto nbt_list_find_value_string(const nbt_list* list, std::int32_t index) noexcept
{
    assert(list->mark != nullptr);
    assert(list->source != nullptr);
    assert(list->element_type == nbt_type::tag_string);
    auto begin{nbt_list_iterator_begin(list)};
    auto end{nbt_list_iterator_end(list)};
    if (index >= list->length)
    {
        return end;
    }
    return nbt_list_find_value_string(index, std::addressof(begin), std::addressof(end));
}

inline  auto nbt_list_find_value_list(const nbt_list* list, std::int32_t index) noexcept
{
    assert(list->mark != nullptr);
    assert(list->source != nullptr);
    assert(list->element_type == nbt_type::tag_list);
    auto begin{nbt_list_iterator_begin(list)};
    auto end{nbt_list_iterator_end(list)};
    if (index >= list->length)
    {
        return end;
    }
    return nbt_list_find_value_list(index, std::addressof(begin), std::addressof(end));
}

inline  auto nbt_list_find_value_compound(const nbt_list* list, std::int32_t index) noexcept
{
    assert(list->mark != nullptr);
    assert(list->source != nullptr);
    assert(list->element_type == nbt_type::tag_compound);
    auto begin{nbt_list_iterator_begin(list)};
    auto end{nbt_list_iterator_end(list)};
    if (index >= list->length)
    {
        return end;
    }
    return nbt_list_find_value_compound(index, std::addressof(begin), std::addressof(end));
}

inline  auto nbt_list_find_value_int_array(const nbt_list* list, std::int32_t index) noexcept
{
    assert(list->mark != nullptr);
    assert(list->source != nullptr);
    assert(list->element_type == nbt_type::tag_int_array);
    auto begin{nbt_list_iterator_begin(list)};
    auto end{nbt_list_iterator_end(list)};
    if (index >= list->length)
    {
        return end;
    }
    return nbt_list_find_value_int_array(index, std::addressof(begin), std::addressof(end));
}

inline  auto nbt_list_find_value_long_array(const nbt_list* list, std::int32_t index) noexcept
{
    assert(list->mark != nullptr);
    assert(list->source != nullptr);
    assert(list->element_type == nbt_type::tag_long_array);
    auto begin{nbt_list_iterator_begin(list)};
    auto end{nbt_list_iterator_end(list)};
    if (index >= list->length)
    {
        return end;
    }
    return nbt_list_find_value_long_array(index, std::addressof(begin), std::addressof(end));
}

}  // namespace nbt_list_function
namespace swapper {

template<typename T>
inline  T& byte_as_type(std::byte* ptr) noexcept
{
    return *reinterpret_cast<T*>(ptr);
}

template<typename T, std::endian nbt_endian>
requires(nbt_endian == std::endian::big || nbt_endian == std::endian::little) inline  T endian_get(std::byte* ptr) noexcept
{
    if constexpr (nbt_endian == std::endian::big)
    {
        return fast_io::big_endian(byte_as_type<std::make_unsigned_t<T>>(ptr));
    }
    else
    {
        return fast_io::little_endian(byte_as_type<std::make_unsigned_t<T>>(ptr));
    }
}

template<typename T, std::endian nbt_endian>
requires(nbt_endian == std::endian::big || nbt_endian == std::endian::little) inline  T endian_make_native_get(std::byte* ptr) noexcept
{
    using unsigned_T = std::make_unsigned_t<T>;
    T& t = byte_as_type<T>(ptr);
    if constexpr (nbt_endian == std::endian::big)
    {
        t = static_cast<T>(fast_io::big_endian(static_cast<unsigned_T>(t)));
    }
    else
    {
        t = static_cast<T>(fast_io::little_endian(static_cast<unsigned_T>(t)));
    }
    return t;
}

template<typename T, std::endian nbt_endian>
requires(nbt_endian == std::endian::big || nbt_endian == std::endian::little) inline  T endian_get_make_native(std::byte* ptr) noexcept
{
    using unsigned_T = std::make_unsigned_t<T>;
    T& t = byte_as_type<T>(ptr);
    T ret = t;
    if constexpr (nbt_endian == std::endian::big)
    {
        t = static_cast<T>(fast_io::big_endian(static_cast<unsigned_T>(t)));
    }
    else
    {
        t = static_cast<T>(fast_io::little_endian(static_cast<unsigned_T>(t)));
    }
    return ret;
}

template<typename T, std::endian nbt_endian>
requires(nbt_endian == std::endian::big || nbt_endian == std::endian::little) inline  void endian_make_native(std::byte* ptr) noexcept
{
    using unsigned_T = std::make_unsigned_t<T>;
    unsigned_T& t = byte_as_type<unsigned_T>(ptr);
    if constexpr (nbt_endian == std::endian::big)
    {
        t = fast_io::big_endian(t);
    }
    else
    {
        t = fast_io::little_endian(t);
    }
    return;
}

struct default_read_swapper
{
    template<std::endian nbt_endian>
    static constexpr inline void tag_byte(std::byte*& current_pos) noexcept
    {
        current_pos += (sizeof(std::int8_t));
    }
    template<std::endian nbt_endian>
    static constexpr inline void tag_short(std::byte*& current_pos) noexcept
    {
        endian_make_native<std::uint16_t, nbt_endian>(current_pos);
        current_pos += (sizeof(std::int16_t));
    }
    template<std::endian nbt_endian>
    static constexpr inline void tag_int(std::byte*& current_pos) noexcept
    {
        endian_make_native<std::uint32_t, nbt_endian>(current_pos);
        current_pos += (sizeof(std::int32_t));
    }
    template<std::endian nbt_endian>
    static constexpr inline void tag_long(std::byte*& current_pos) noexcept
    {
        endian_make_native<std::uint64_t, nbt_endian>(current_pos);
        current_pos += (sizeof(std::int64_t));
    }
    template<std::endian nbt_endian>
    static constexpr inline void tag_float(std::byte*& current_pos) noexcept
    {
        endian_make_native<std::uint32_t, nbt_endian>(current_pos);
        current_pos += (sizeof(std::float_t));
    }
    template<std::endian nbt_endian>
    static constexpr inline void tag_double(std::byte*& current_pos) noexcept
    {
        endian_make_native<std::uint64_t, nbt_endian>(current_pos);
        current_pos += (sizeof(std::double_t));
    }
    template<std::endian nbt_endian>
    static constexpr inline void tag_byte_array(std::byte*& current_pos) noexcept
    {
        auto len = endian_make_native_get<std::int32_t, nbt_endian>(current_pos);
        current_pos += sizeof(std::int32_t);
        current_pos += sizeof(std::int8_t) * len;
    }
    template<std::endian nbt_endian>
    static constexpr inline void tag_string(std::byte*& current_pos) noexcept
    {
        auto len = endian_make_native_get<std::uint16_t, nbt_endian>(current_pos);
        current_pos += sizeof(std::uint16_t);
        current_pos += sizeof(char) * len;
    }
    template<std::endian nbt_endian>
    static constexpr inline void tag_int_array(std::byte*& current_pos) noexcept
    {
        auto len = endian_make_native_get<std::int32_t, nbt_endian>(current_pos);
        current_pos += sizeof(std::int32_t);
        if constexpr (nbt_endian == std::endian::native)
        {
            current_pos += sizeof(std::int32_t) * len;
        }
        else
        {
            for (std::int32_t _index = 0; _index < len; _index++)
            {
                endian_make_native<std::int32_t, nbt_endian>(current_pos);
                current_pos += sizeof(std::int32_t);
            }
        }
    }
    template<std::endian nbt_endian>
    static constexpr inline void tag_long_array(std::byte*& current_pos) noexcept
    {
        auto len = endian_make_native_get<std::int32_t, nbt_endian>(current_pos);
        current_pos += sizeof(std::int32_t);
        current_pos += sizeof(std::int32_t);
        if constexpr (nbt_endian == std::endian::native)
        {
            current_pos += sizeof(std::int64_t) * len;
        }
        else
        {
            for (std::int32_t _index = 0; _index < len; _index++)
            {
                endian_make_native<std::int64_t, nbt_endian>(current_pos);
                current_pos += sizeof(std::int64_t);
            }
        }
    }
};

struct default_write_swapper
{
    template<std::endian nbt_endian>
    static constexpr inline void tag_byte(std::byte*& current_pos) noexcept
    {
        current_pos += (sizeof(std::int8_t));
    }
    template<std::endian nbt_endian>
    static constexpr inline void tag_short(std::byte*& current_pos) noexcept
    {
        endian_make_native<std::uint16_t, nbt_endian>(current_pos);
        current_pos += (sizeof(std::int16_t));
    }
    template<std::endian nbt_endian>
    static constexpr inline void tag_int(std::byte*& current_pos) noexcept
    {
        endian_make_native<std::uint32_t, nbt_endian>(current_pos);
        current_pos += (sizeof(std::int32_t));
    }
    template<std::endian nbt_endian>
    static constexpr inline void tag_long(std::byte*& current_pos) noexcept
    {
        endian_make_native<std::uint64_t, nbt_endian>(current_pos);
        current_pos += (sizeof(std::int64_t));
    }
    template<std::endian nbt_endian>
    static constexpr inline void tag_float(std::byte*& current_pos) noexcept
    {
        endian_make_native<std::uint32_t, nbt_endian>(current_pos);
        current_pos += (sizeof(std::float_t));
    }
    template<std::endian nbt_endian>
    static constexpr inline void tag_double(std::byte*& current_pos) noexcept
    {
        endian_make_native<std::uint64_t, nbt_endian>(current_pos);
        current_pos += (sizeof(std::double_t));
    }
    template<std::endian nbt_endian>
    static constexpr inline void tag_byte_array(std::byte*& current_pos) noexcept
    {
        auto len = endian_get_make_native<std::int32_t, nbt_endian>(current_pos);
        current_pos += sizeof(std::int32_t);
        current_pos += sizeof(std::int8_t) * len;
    }
    template<std::endian nbt_endian>
    static constexpr inline void tag_string(std::byte*& current_pos) noexcept
    {
        auto len = endian_get_make_native<std::uint16_t, nbt_endian>(current_pos);
        current_pos += sizeof(std::uint16_t);
        current_pos += sizeof(char) * len;
    }
    template<std::endian nbt_endian>
    static constexpr inline void tag_int_array(std::byte*& current_pos) noexcept
    {
        auto len = endian_get_make_native<std::int32_t, nbt_endian>(current_pos);
        current_pos += sizeof(std::int32_t);
        //current_pos += sizeof(std::int32_t) * len;
        for (std::int32_t _index = 0; _index < len; _index++)
        {
            endian_make_native<std::int32_t, nbt_endian>(current_pos);
            current_pos += sizeof(std::int32_t);
        }
    }
    template<std::endian nbt_endian>
    static constexpr inline void tag_long_array(std::byte*& current_pos) noexcept
    {
        auto len = endian_get_make_native<std::int32_t, nbt_endian>(current_pos);
        current_pos += sizeof(std::int32_t);
        //current_pos += sizeof(std::int64_t) * len;
        for (std::int32_t _index = 0; _index < len; _index++)
        {
            endian_make_native<std::int64_t, nbt_endian>(current_pos);
            current_pos += sizeof(std::int64_t);
        }
    }
};

}  // namespace swapper
namespace read_write {

template<bool in_place = true, bool bound_check = false, std::endian nbt_endian = std::endian::big, swapper::read_swapper rswap = swapper::default_read_swapper>
[[nodiscard]] inline  auto read(std::byte* source, std::size_t source_len) -> nbt_document
{
#define bound_check_return(pos)             \
    if ((pos) > source_len) [[unlikely]]    \
    {                                       \
        throw nbt_parse_error::end_of_file; \
    }

    nbt_document t{};
    if constexpr (in_place)
    {
        t.source_m = nullptr;
        t.source = source;
        t.source_len = source_len;
    }
    else
    {
        t.source_m = static_cast<std::byte*>(fast_io::native_global_allocator::allocate(source_len));
        t.source = t.source_m;
        t.source_len = source_len;
        fast_io::freestanding::non_overlapped_copy_n(source, source_len, t.source);
    }
    auto src{t.source};
    std::size_t readed_length{0};

    auto current_pos{src + 1};  //read first byte (id), +1
    if constexpr (bound_check)
    {
        readed_length += 1;
        bound_check_return(readed_length);
    }
    if (swapper::endian_get<std::uint8_t, nbt_endian>(src) != 0) [[likely]]  //is tag_end, ignore tag name and payload
    {
        //read tag name
        {
            if constexpr (bound_check)
            {
                readed_length += sizeof(std::uint16_t);
                bound_check_return(readed_length);
                auto len = swapper::endian_get<std::uint16_t, nbt_endian>(src + 1);
                readed_length += len;
                bound_check_return(readed_length);
            }
            std::uint16_t len = swapper::endian_make_native_get<std::uint16_t, nbt_endian>(src + 1);  //read name length
            current_pos += sizeof(std::uint16_t);
            current_pos += len;  // read name, +2 +length*1
        }
        switch (swapper::endian_get<std::uint8_t, nbt_endian>(src))
        {
            case 1:  //tag_byte
                if constexpr (bound_check)
                {
                    readed_length += 1;
                    bound_check_return(readed_length);
                }
                rswap::template tag_byte<nbt_endian>(current_pos);
                break;
            case 2:  //tag_short
                if constexpr (bound_check)
                {
                    readed_length += 2;
                    bound_check_return(readed_length);
                }
                rswap::template tag_short<nbt_endian>(current_pos);
                break;
            case 3:  //tag_int
                if constexpr (bound_check)
                {
                    readed_length += 4;
                    bound_check_return(readed_length);
                }
                rswap::template tag_int<nbt_endian>(current_pos);
                break;
            case 4:  //tag_long
                if constexpr (bound_check)
                {
                    readed_length += 8;
                    bound_check_return(readed_length);
                }
                rswap::template tag_long<nbt_endian>(current_pos);
                break;
            case 5:  //tag_float
                if constexpr (bound_check)
                {
                    readed_length += 4;
                    bound_check_return(readed_length);
                }
                rswap::template tag_float<nbt_endian>(current_pos);
                break;
            case 6:  //tag_double
                if constexpr (bound_check)
                {
                    readed_length += 8;
                    bound_check_return(readed_length);
                }
                rswap::template tag_double<nbt_endian>(current_pos);
                break;
            case 7:  //tag_byte_array
                if constexpr (bound_check)
                {
                    readed_length += 4;
                    bound_check_return(readed_length);
                    auto len = swapper::endian_get<std::int32_t, nbt_endian>(current_pos);
                    readed_length += len;
                    bound_check_return(readed_length);
                }
                rswap::template tag_byte_array<nbt_endian>(current_pos);
                break;
            case 8:  //tag_string

                if constexpr (bound_check)
                {
                    readed_length += 2;
                    bound_check_return(readed_length);
                    auto len = swapper::endian_get<std::uint16_t, nbt_endian>(current_pos);
                    readed_length += len;
                    bound_check_return(readed_length);
                }
                rswap::template tag_string<nbt_endian>(current_pos);
                break;
            case 9:  //tag_list
                goto general_start;
            case 10:  //tag_compound
                goto general_start;
            case 11:  //tag_int_array
                if constexpr (bound_check)
                {
                    readed_length += 4;
                    bound_check_return(readed_length);
                    auto len = swapper::endian_get<std::int32_t, nbt_endian>(current_pos);
                    readed_length += len * sizeof(std::int32_t);
                    bound_check_return(readed_length);
                }
                rswap::template tag_int_array<nbt_endian>(current_pos);
                break;
            case 12:  //tag_long_array
                if constexpr (bound_check)
                {
                    readed_length += 4;
                    bound_check_return(readed_length);
                    auto len = swapper::endian_get<std::int32_t, nbt_endian>(current_pos);
                    readed_length += len * sizeof(std::int64_t);
                    bound_check_return(readed_length);
                }
                rswap::template tag_long_array<nbt_endian>(current_pos);
                break;
            default:
                throw nbt_parse_error::invalid;
        }
    }
    t.mark_m = nullptr;
    t.mark = t.mark_m;
    t.mark_len = 0;
    return t;
general_start:
    using mark = mark_t;
    std::size_t alc_len{static_cast<std::size_t>(t.source_len) / 32 /*todo: magic number*/ + 4 /*avoid zero*/};
    mark* mark_hdr{static_cast<mark*>(fast_io::native_global_allocator::allocate(alc_len * sizeof(mark)))};  //first element of mark array
    mark* mark_end{mark_hdr + alc_len};                                                                      //end+1 of mark array
    mark* use_end{mark_hdr};                                                                                 //last element used
    mark* _mark_tmp{nullptr};                                                                                //tmp
    mark* current{use_end};                                                                                  //current container
    mark* parent{use_end};                                                                                   //parent container

    current->cache.general_parrent_offset = 0;

    if (swapper::endian_get<std::uint8_t, nbt_endian>(src) == 9)  //list
    {
        goto list_general_begin;
    }
    else  //compound
    {
        goto comp_general_begin;
    }

comp_begin:
    parent = current;
    use_end++;
    if (use_end > mark_end) [[unlikely]]
    {
        alc_len += alc_len / 2;
        if (std::is_constant_evaluated())
        {
            _mark_tmp = reinterpret_cast<mark*>(fast_io::native_global_allocator::reallocate_n(mark_hdr, (mark_end - mark_hdr) * sizeof(mark), alc_len * sizeof(mark)));
        }
        else
        {
            _mark_tmp = reinterpret_cast<mark*>(fast_io::native_global_allocator::reallocate(mark_hdr, alc_len * sizeof(mark)));  //reallocate should make sure that the allocation is successful.
        }
        use_end = _mark_tmp + (use_end - mark_hdr);
        parent = _mark_tmp + (parent - mark_hdr);
        mark_hdr = _mark_tmp;
        mark_end = _mark_tmp + alc_len;
    }
    current = use_end;
comp_general_begin:
    current->cache.general_parrent_offset = static_cast<std::uint32_t>(use_end - parent);
    current->cache.general_is_compound = 1;
    // goto comp_item_begin
comp_item_begin:
{
    if constexpr (bound_check)
    {
        readed_length++;
        bound_check_return(readed_length);
    }
    const std::uint8_t id{swapper::endian_get<std::uint8_t, nbt_endian>(current_pos)};
    current_pos++;
    if (id == 0) [[unlikely]]
    {
        goto comp_end;
    }
    if constexpr (bound_check)
    {
        readed_length += sizeof(std::uint16_t);
        bound_check_return(readed_length);
        auto len = swapper::endian_get<std::uint16_t, nbt_endian>(current_pos);
        readed_length += len;
        bound_check_return(readed_length);
    }
    auto len = swapper::endian_make_native_get<std::uint16_t, nbt_endian>(current_pos);
    current_pos += sizeof(std::uint16_t);
    current_pos += (len * sizeof(char));
    switch (id)
    {
        case 1:
        {
            if constexpr (bound_check)
            {
                readed_length += 1;
                bound_check_return(readed_length);
            }
            rswap::template tag_byte<nbt_endian>(current_pos);
            break;
        }
        case 2:
        {
            if constexpr (bound_check)
            {
                readed_length += 2;
                bound_check_return(readed_length);
            }
            rswap::template tag_short<nbt_endian>(current_pos);
            break;
        }
        case 3:
        {
            if constexpr (bound_check)
            {
                readed_length += 4;
                bound_check_return(readed_length);
            }
            rswap::template tag_int<nbt_endian>(current_pos);
            break;
        }
        case 4:
        {
            if constexpr (bound_check)
            {
                readed_length += 8;
                bound_check_return(readed_length);
            }
            rswap::template tag_long<nbt_endian>(current_pos);
            break;
        }
        case 5:
        {
            if constexpr (bound_check)
            {
                readed_length += 4;
                bound_check_return(readed_length);
            }
            rswap::template tag_float<nbt_endian>(current_pos);
            break;
        }
        case 6:
        {
            if constexpr (bound_check)
            {
                readed_length += 8;
                bound_check_return(readed_length);
            }
            rswap::template tag_double<nbt_endian>(current_pos);
            break;
        }
        case 7:
        {
            if constexpr (bound_check)
            {
                readed_length += 4;
                bound_check_return(readed_length);
                auto arrlen = swapper::endian_get<std::int32_t, nbt_endian>(current_pos);
                readed_length += arrlen;
                bound_check_return(readed_length);
            }
            rswap::template tag_byte_array<nbt_endian>(current_pos);
            break;
        }
        case 8:
        {
            if constexpr (bound_check)
            {
                readed_length += 2;
                bound_check_return(readed_length);
                auto arrlen = swapper::endian_get<std::uint16_t, nbt_endian>(current_pos);
                readed_length += arrlen;
                bound_check_return(readed_length);
            }
            rswap::template tag_string<nbt_endian>(current_pos);
            break;
        }
        case 9:
        {
            goto list_begin;
        }
        case 10:
        {
            goto comp_begin;
        }
        case 11:
        {
            if constexpr (bound_check)
            {
                readed_length += 4;
                bound_check_return(readed_length);
                std::size_t arrlen = swapper::endian_get<std::int32_t, nbt_endian>(current_pos);
                readed_length += arrlen * sizeof(std::int32_t);
                bound_check_return(readed_length);
            }
            rswap::template tag_int_array<nbt_endian>(current_pos);
            break;
        }
        case 12:
        {
            if constexpr (bound_check)
            {
                readed_length += 4;
                bound_check_return(readed_length);
                std::size_t arrlen = swapper::endian_get<std::int32_t, nbt_endian>(current_pos);
                readed_length += arrlen * sizeof(std::int64_t);
                bound_check_return(readed_length);
            }
            rswap::template tag_long_array<nbt_endian>(current_pos);
            break;
        }
        default:
            fast_io::native_global_allocator::deallocate_n(mark_hdr, alc_len * sizeof(mark));
            throw nbt_parse_error::invalid;
    };
    goto comp_item_begin;
}
comp_end:
    if (current->cache.general_parrent_offset == 0) [[unlikely]]
    {
        current->store.end = current_pos;
        current->store.flat_next_mark = use_end - current + 1;
        goto read_finish;
    }
    current->store.end = current_pos;
    current->store.flat_next_mark = use_end - current + 1;
    current = parent;
    parent = parent - (parent->cache.general_parrent_offset);
    if (current->cache.general_is_compound == 1)
    {
        goto comp_item_begin;
    }
    else
    {
        goto list_item_begin;
    }
list_begin:
    parent = current;
    use_end++;
    if (use_end > mark_end) [[unlikely]]
    {
        alc_len += alc_len / 2;
        if (std::is_constant_evaluated())
        {
            #ifdef use_fastio
            _mark_tmp = reinterpret_cast<mark*>(fast_io::native_global_allocator::reallocate_n(mark_hdr, (mark_end - mark_hdr) * sizeof(mark), alc_len * sizeof(mark)));
            #else
            _mark_tmp = reinterpret_cast<mark*>(realloc(mark_hdr, (mark_end - mark_hdr) * sizeof(mark), alc_len * sizeof(mark)));
            #endif
        }
        else
        {
            _mark_tmp = reinterpret_cast<mark*>(fast_io::native_global_allocator::reallocate(mark_hdr, alc_len * sizeof(mark)));  //reallocate should make sure that the allocation is successful.
        }
        use_end = _mark_tmp + (use_end - mark_hdr);
        parent = _mark_tmp + (parent - mark_hdr);
        mark_hdr = _mark_tmp;
        mark_end = _mark_tmp + alc_len;
    }
    current = use_end;
list_general_begin:
    if constexpr (bound_check)
    {
        readed_length++;
        readed_length += sizeof(std::uint32_t);
        bound_check_return(readed_length);
        auto id = swapper::endian_get<std::uint8_t, nbt_endian>(current_pos);
        if (id >= 0 && id <= 6)
        {
            const std::size_t len = swapper::endian_get<std::uint32_t, nbt_endian>(current_pos + 1);
            switch (id)
            {
                case 0:
                    break;
                case 1:
                    readed_length += len;
                    break;
                case 2:
                    readed_length += len * 2;
                    break;
                case 3:
                    readed_length += len * 4;
                    break;
                case 4:
                    readed_length += len * 8;
                    break;
                case 5:
                    readed_length += len * 4;
                    break;
                case 6:
                    readed_length += len * 8;
                    break;
                default:
                    break;
            }
            bound_check_return(readed_length);
        }
    }
    current->cache.list_type = swapper::endian_get<std::uint8_t, nbt_endian>(current_pos);
    current->cache.general_parrent_offset = static_cast<std::uint32_t>(current - parent);
    current->cache.general_is_compound = 0;
    current_pos++;
    current->cache.list_total_length = swapper::endian_make_native_get<std::uint32_t, nbt_endian>(current_pos);
    current->cache.list_current_length = 0;
    current_pos += sizeof(std::uint32_t);
    goto list_item_begin;
list_item_begin:
    if (current->cache.list_current_length >= current->cache.list_total_length) [[unlikely]]
    {
        goto list_end;
    }

    current->cache.list_current_length++;
    switch (current->cache.list_type)
    {
        case 0:
        {
            break;
        }
        case 1:
        {
            rswap::template tag_byte<nbt_endian>(current_pos);
            break;
        }
        case 2:
        {
            rswap::template tag_short<nbt_endian>(current_pos);
            break;
        }
        case 3:
        {
            rswap::template tag_int<nbt_endian>(current_pos);
            break;
        }
        case 4:
        {
            rswap::template tag_long<nbt_endian>(current_pos);
            break;
        }
        case 5:
        {
            rswap::template tag_float<nbt_endian>(current_pos);
            break;
        }
        case 6:
        {
            rswap::template tag_double<nbt_endian>(current_pos);
            break;
        }
        case 7:
        {
            if constexpr (bound_check)
            {
                readed_length += 4;
                bound_check_return(readed_length);
                auto len = swapper::endian_get<std::int32_t, nbt_endian>(current_pos);
                readed_length += len;
                bound_check_return(readed_length);
            }
            rswap::template tag_byte_array<nbt_endian>(current_pos);
            break;
        }
        case 8:
        {
            if constexpr (bound_check)
            {
                readed_length += 2;
                bound_check_return(readed_length);
                auto len = swapper::endian_get<std::uint16_t, nbt_endian>(current_pos);
                readed_length += len;
                bound_check_return(readed_length);
            }
            rswap::template tag_string<nbt_endian>(current_pos);
            break;
        }
        case 9:
        {
            goto list_begin;
        }
        case 10:
        {
            goto comp_begin;
        }
        case 11:
        {
            if constexpr (bound_check)
            {
                readed_length += 4;
                bound_check_return(readed_length);
                auto len = swapper::endian_get<std::int32_t, nbt_endian>(current_pos);
                readed_length += len * sizeof(std::int32_t);
                bound_check_return(readed_length);
            }
            rswap::template tag_int_array<nbt_endian>(current_pos);
            break;
        }
        case 12:
        {
            if constexpr (bound_check)
            {
                readed_length += 4;
                bound_check_return(readed_length);
                auto len = swapper::endian_get<std::int32_t, nbt_endian>(current_pos);
                readed_length += len * sizeof(std::int64_t);
                bound_check_return(readed_length);
            }
            rswap::template tag_long_array<nbt_endian>(current_pos);
            break;
        }
        default:
            fast_io::native_global_allocator::deallocate_n(mark_hdr, alc_len * sizeof(mark));
            throw nbt_parse_error::invalid;
    };
    goto list_item_begin;
list_end:
    if (current->cache.general_parrent_offset == 0) [[unlikely]]
    {
        current->store.end = current_pos;
        current->store.flat_next_mark = use_end - current + 1;
        goto read_finish;
    }
    current->store.end = current_pos;
    current->store.flat_next_mark = use_end - current + 1;
    current = parent;
    parent = parent - (parent->cache.general_parrent_offset);
    if (current->cache.general_is_compound == 1)
    {
        goto comp_item_begin;
    }
    else
    {
        goto list_item_begin;
    }

read_finish:
    t.mark_m = mark_hdr;
    t.mark = t.mark_m;
    t.mark_len = (mark_end - mark_hdr);
    return t;

#undef bound_check_return
}
}  // namespace read_write
}  // namespace impl

using nbt_parse_error = impl::nbt_parse_error;
using nbt_type = impl::nbt_type;
struct nbt_type_error
{
    nbt_type is;
    nbt_type as;
};
class nbt_document;
template<nbt_type tag_type, nbt_type list_element_type>
class nbt_any_tag;
template<nbt_type list_element_type>
class nbt_list;
class nbt_compound;

template<bool in_place, bool bound_check, std::endian nbt_endian, impl::swapper::read_swapper rswap>
[[nodiscard]] inline  auto read(std::span<std::byte, std::dynamic_extent> source) -> nbt_document;

template<nbt_type list_element_type>
class nbt_list
{
  private:
    impl::nbt_list impl_list;

    template<nbt_type tag_type, nbt_type list_element_type_>
    friend class nbt_any_tag;

  public:
    class iterator
    {
      private:
        impl::nbt_list::iterator impl_iterator;

      public:
        inline  iterator& operator++() noexcept
        {
            if constexpr (list_element_type == nbt_type::tag_end)
            {
                impl::nbt_list_function::nbt_list_iterator_next_end(std::addressof(impl_iterator));
            }
            else if constexpr (list_element_type == nbt_type::tag_byte)
            {
                impl::nbt_list_function::nbt_list_iterator_next_byte(std::addressof(impl_iterator));
            }
            else if constexpr (list_element_type == nbt_type::tag_short)
            {
                impl::nbt_list_function::nbt_list_iterator_next_short(std::addressof(impl_iterator));
            }
            else if constexpr (list_element_type == nbt_type::tag_int)
            {
                impl::nbt_list_function::nbt_list_iterator_next_int(std::addressof(impl_iterator));
            }
            else if constexpr (list_element_type == nbt_type::tag_long)
            {
                impl::nbt_list_function::nbt_list_iterator_next_long(std::addressof(impl_iterator));
            }
            else if constexpr (list_element_type == nbt_type::tag_float)
            {
                impl::nbt_list_function::nbt_list_iterator_next_float(std::addressof(impl_iterator));
            }
            else if constexpr (list_element_type == nbt_type::tag_double)
            {
                impl::nbt_list_function::nbt_list_iterator_next_double(std::addressof(impl_iterator));
            }
            else if constexpr (list_element_type == nbt_type::tag_byte_array)
            {
                impl::nbt_list_function::nbt_list_iterator_next_byte_array(std::addressof(impl_iterator));
            }
            else if constexpr (list_element_type == nbt_type::tag_string)
            {
                impl::nbt_list_function::nbt_list_iterator_next_string(std::addressof(impl_iterator));
            }
            else if constexpr (list_element_type == nbt_type::tag_list)
            {
                impl::nbt_list_function::nbt_list_iterator_next_list(std::addressof(impl_iterator));
            }
            else if constexpr (list_element_type == nbt_type::tag_compound)
            {
                impl::nbt_list_function::nbt_list_iterator_next_compound(std::addressof(impl_iterator));
            }
            else if constexpr (list_element_type == nbt_type::tag_int_array)
            {
                impl::nbt_list_function::nbt_list_iterator_next_int_array(std::addressof(impl_iterator));
            }
            else
            {
                impl::nbt_list_function::nbt_list_iterator_next_long_array(std::addressof(impl_iterator));
            }
            return *this;
        }
        inline  const iterator operator++(int) noexcept
        {
            auto old = *this;
            ++(*this);
            return old;
        }
        [[nodiscard]] inline  bool operator==(const iterator& x) const noexcept
        {
            return impl::nbt_list_function::nbt_list_iter_equal(std::addressof(impl_iterator), std::addressof(x.impl_iterator));
        }
        [[nodiscard]] inline  bool operator!=(const iterator& x) const noexcept
        {
            return !impl::nbt_list_function::nbt_list_iter_equal(std::addressof(impl_iterator), std::addressof(x.impl_iterator));
        }

        template<nbt_type element_type = nbt_type::tag_end>
        [[nodiscard]] inline  auto value() const
        {
            nbt_any_tag<list_element_type, element_type> tag{};
            if constexpr (list_element_type == nbt_type::tag_list)
            {
                auto list_type{
                    static_cast<nbt_type>(*impl_iterator.source)};
                if (list_type != element_type)
                {
                    throw nbt_type_error{.is = list_type, .as = element_type};
                }
            }
            tag.impl_any_tag = impl::nbt_list_function::nbt_list_iter_value(std::addressof(impl_iterator));
            return tag.get();
        }

        [[nodiscard]] inline consteval nbt_type type() const noexcept
        {
            return list_element_type;
        }

        [[nodiscard]] inline  nbt_type element_type() const noexcept
        {
            return static_cast<nbt_type>(*impl_iterator.source);
        }

        friend class nbt_list<list_element_type>;
    };

    [[nodiscard]] inline  iterator begin() const noexcept
    {
        iterator iter{};
        iter.impl_iterator = impl::nbt_list_function::nbt_list_iterator_begin(std::addressof(impl_list));
        return iter;
    }

    [[nodiscard]] inline  iterator end() const noexcept
    {
        iterator iter{};
        iter.impl_iterator = impl::nbt_list_function::nbt_list_iterator_end(std::addressof(impl_list));
        return iter;
    }
    [[nodiscard]] inline  std::size_t size() const noexcept
    {
        return impl_list.length;
    }
    [[nodiscard]] inline  auto operator[](std::int32_t index) const noexcept
    {
        iterator iter{};
        if constexpr (list_element_type == nbt_type::tag_end)
        {
            iter.impl_iterator = (impl::nbt_list_function::nbt_list_find_value_end(std::addressof(impl_list), index));
        }
        else if constexpr (list_element_type == nbt_type::tag_byte)
        {
            iter.impl_iterator = (impl::nbt_list_function::nbt_list_find_value_byte(std::addressof(impl_list), index));
        }
        else if constexpr (list_element_type == nbt_type::tag_short)
        {
            iter.impl_iterator = (impl::nbt_list_function::nbt_list_find_value_short(std::addressof(impl_list), index));
        }
        else if constexpr (list_element_type == nbt_type::tag_int)
        {
            iter.impl_iterator = (impl::nbt_list_function::nbt_list_find_value_int(std::addressof(impl_list), index));
        }
        else if constexpr (list_element_type == nbt_type::tag_long)
        {
            iter.impl_iterator = (impl::nbt_list_function::nbt_list_find_value_long(std::addressof(impl_list), index));
        }
        else if constexpr (list_element_type == nbt_type::tag_float)
        {
            iter.impl_iterator = (impl::nbt_list_function::nbt_list_find_value_float(std::addressof(impl_list), index));
        }
        else if constexpr (list_element_type == nbt_type::tag_double)
        {
            iter.impl_iterator = (impl::nbt_list_function::nbt_list_find_value_double(std::addressof(impl_list), index));
        }
        else if constexpr (list_element_type == nbt_type::tag_byte_array)
        {
            iter.impl_iterator = (impl::nbt_list_function::nbt_list_find_value_byte_array(std::addressof(impl_list), index));
        }
        else if constexpr (list_element_type == nbt_type::tag_string)
        {
            iter.impl_iterator = (impl::nbt_list_function::nbt_list_find_value_string(std::addressof(impl_list), index));
        }
        else if constexpr (list_element_type == nbt_type::tag_list)
        {
            iter.impl_iterator = (impl::nbt_list_function::nbt_list_find_value_list(std::addressof(impl_list), index));
        }
        else if constexpr (list_element_type == nbt_type::tag_compound)
        {
            iter.impl_iterator = (impl::nbt_list_function::nbt_list_find_value_compound(std::addressof(impl_list), index));
        }
        else if constexpr (list_element_type == nbt_type::tag_int_array)
        {
            iter.impl_iterator = (impl::nbt_list_function::nbt_list_find_value_int_array(std::addressof(impl_list), index));
        }
        else
        {
            iter.impl_iterator = (impl::nbt_list_function::nbt_list_find_value_long_array(std::addressof(impl_list), index));
        }
        return iter;
    }
};

class nbt_compound
{
  private:
    impl::nbt_compound impl_compound;

    template<nbt_type tag_type, nbt_type list_element_type>
    friend class nbt_any_tag;

  public:
    class iterator
    {
      private:
        impl::nbt_compound::iterator impl_iterator;

      public:
        inline  iterator& operator++() noexcept
        {
            impl::nbt_compound_function::nbt_compound_iterator_next(std::addressof(impl_iterator));
            return *this;
        }
        inline  const iterator operator++(int) noexcept
        {
            auto old = *this;
            ++(*this);
            return old;
        }
        [[nodiscard]] inline  bool operator==(const iterator& x) const noexcept
        {
            return impl::nbt_compound_function::nbt_compound_iter_equal(std::addressof(impl_iterator), std::addressof(x.impl_iterator));
        }
        [[nodiscard]] inline  bool operator!=(const iterator& x) const noexcept
        {
            return !impl::nbt_compound_function::nbt_compound_iter_equal(std::addressof(impl_iterator), std::addressof(x.impl_iterator));
        }

        template<nbt_type tag_type, nbt_type list_element_type = nbt_type::tag_end>
        [[nodiscard]] inline  auto value() const
        {
            auto real_type{static_cast<nbt_type>(impl::swapper::byte_as_type<std::uint8_t>(impl_iterator.source))};
            if (real_type != tag_type)
            {
                throw nbt_type_error{.is = real_type, .as = tag_type};
            }
            if constexpr (tag_type == nbt_type::tag_list)
            {
                auto list_type{
                    static_cast<nbt_type>(impl::swapper::byte_as_type<std::uint8_t>(
                        impl_iterator.source + sizeof(std::uint8_t) + sizeof(std::uint16_t) +
                        impl::swapper::byte_as_type<std::uint16_t>(
                            impl_iterator.source + sizeof(std::uint8_t))))};
                if (list_type != list_element_type)
                {
                    throw nbt_type_error{.is = list_type, .as = list_element_type};
                }
            }
            nbt_any_tag<tag_type, list_element_type> tag{};
            tag.impl_any_tag = impl::nbt_compound_function::nbt_compound_iter_value(std::addressof(impl_iterator));
            return tag.get();
        }

        [[nodiscard]] inline  std::u8string_view key() const noexcept
        {
            return impl::nbt_compound_function::nbt_compound_iter_key(std::addressof(impl_iterator));
        }

        [[nodiscard]] inline  nbt_type type() const noexcept
        {
            return static_cast<nbt_type>(impl::swapper::byte_as_type<std::uint8_t>(impl_iterator.source));
        }

        [[nodiscard]] inline  nbt_type element_type() const noexcept
        {
            return static_cast<nbt_type>(impl::swapper::byte_as_type<std::uint8_t>(
                impl_iterator.source + sizeof(std::uint8_t) + sizeof(std::uint16_t) +
                impl::swapper::byte_as_type<std::uint16_t>(
                    impl_iterator.source + sizeof(std::uint8_t))));
        }

        using difference_type = std::ptrdiff_t;

        friend class nbt_compound;
    };

    [[nodiscard]] inline  iterator begin() const noexcept
    {
        iterator iter{};
        iter.impl_iterator = impl::nbt_compound_function::nbt_compound_iterator_begin(std::addressof(impl_compound));
        return iter;
    }

    [[nodiscard]] inline  iterator end() const noexcept
    {
        iterator iter{};
        iter.impl_iterator = impl::nbt_compound_function::nbt_compound_iterator_end(std::addressof(impl_compound));
        return iter;
    }

    [[nodiscard]] inline  iterator at(std::u8string_view key) const
    {
        iterator iter{};
        iter.impl_iterator = impl::nbt_compound_function::nbt_compound_find_value(std::addressof(impl_compound), key);
        return iter;
    }
};

template<nbt_type tag_type, nbt_type list_element_type = nbt_type::tag_end>
class nbt_any_tag
{
  private:
    impl::any_tag impl_any_tag;

    friend class nbt_document;
    template<nbt_type list_element_type_>
    friend class nbt_list;
    friend class nbt_compound;

  public:
    [[nodiscard]] inline consteval nbt_type type() noexcept
    {
        return tag_type;
    }

    [[nodiscard]] inline consteval nbt_type element_type() noexcept
    {
        return list_element_type;
    }

    [[nodiscard]] inline  auto get() noexcept
    {
        if constexpr (tag_type == nbt_type::tag_end)
        {
            return;
        }
        else if constexpr (tag_type == nbt_type::tag_byte)
        {
            return impl::any_tag_function::any_tag_get_byte(std::addressof(impl_any_tag));
        }
        else if constexpr (tag_type == nbt_type::tag_short)
        {
            return impl::any_tag_function::any_tag_get_short(std::addressof(impl_any_tag));
        }
        else if constexpr (tag_type == nbt_type::tag_int)
        {
            return impl::any_tag_function::any_tag_get_int(std::addressof(impl_any_tag));
        }
        else if constexpr (tag_type == nbt_type::tag_long)
        {
            return impl::any_tag_function::any_tag_get_long(std::addressof(impl_any_tag));
        }
        else if constexpr (tag_type == nbt_type::tag_float)
        {
            return impl::any_tag_function::any_tag_get_float(std::addressof(impl_any_tag));
        }
        else if constexpr (tag_type == nbt_type::tag_double)
        {
            return impl::any_tag_function::any_tag_get_double(std::addressof(impl_any_tag));
        }
        else if constexpr (tag_type == nbt_type::tag_byte_array)
        {
            return impl::any_tag_function::any_tag_get_byte_array(std::addressof(impl_any_tag));
        }
        else if constexpr (tag_type == nbt_type::tag_string)
        {
            return impl::any_tag_function::any_tag_get_string(std::addressof(impl_any_tag));
        }
        else if constexpr (tag_type == nbt_type::tag_list)
        {
            if constexpr (list_element_type == nbt_type::tag_end)
            {
                return;
            }
            else if constexpr (list_element_type == nbt_type::tag_byte)
            {
                return impl::any_tag_function::any_tag_get_list_byte(std::addressof(impl_any_tag));
            }
            else if constexpr (list_element_type == nbt_type::tag_short)
            {
                return impl::any_tag_function::any_tag_get_list_short(std::addressof(impl_any_tag));
            }
            else if constexpr (list_element_type == nbt_type::tag_int)
            {
                return impl::any_tag_function::any_tag_get_list_int(std::addressof(impl_any_tag));
            }
            else if constexpr (list_element_type == nbt_type::tag_long)
            {
                return impl::any_tag_function::any_tag_get_list_long(std::addressof(impl_any_tag));
            }
            else if constexpr (list_element_type == nbt_type::tag_float)
            {
                return impl::any_tag_function::any_tag_get_list_float(std::addressof(impl_any_tag));
            }
            else if constexpr (list_element_type == nbt_type::tag_double)
            {
                return impl::any_tag_function::any_tag_get_list_double(std::addressof(impl_any_tag));
            }
            else if constexpr (list_element_type == nbt_type::tag_byte_array)
            {
                nbt_list<list_element_type> list{};
                list.impl_list = impl::any_tag_function::any_tag_get_list_byte_array(std::addressof(impl_any_tag));
                return list;
            }
            else if constexpr (list_element_type == nbt_type::tag_string)
            {
                nbt_list<list_element_type> list{};
                list.impl_list = impl::any_tag_function::any_tag_get_list_string(std::addressof(impl_any_tag));
                return list;
            }
            else if constexpr (list_element_type == nbt_type::tag_list)
            {
                nbt_list<list_element_type> list{};
                list.impl_list = impl::any_tag_function::any_tag_get_list_list(std::addressof(impl_any_tag));
                return list;
            }
            else if constexpr (list_element_type == nbt_type::tag_compound)
            {
                nbt_list<list_element_type> list{};
                list.impl_list = impl::any_tag_function::any_tag_get_list_compound(std::addressof(impl_any_tag));
                return list;
            }
            else if constexpr (list_element_type == nbt_type::tag_int_array)
            {
                nbt_list<list_element_type> list{};
                list.impl_list = impl::any_tag_function::any_tag_get_list_int_array(std::addressof(impl_any_tag));
                return list;
            }
            else
            {
                nbt_list<list_element_type> list{};
                list.impl_list = impl::any_tag_function::any_tag_get_list_long_array(std::addressof(impl_any_tag));
                return list;
            }
        }
        else if constexpr (tag_type == nbt_type::tag_compound)
        {
            nbt_compound comp{};
            comp.impl_compound = impl::any_tag_function::any_tag_get_compound(std::addressof(impl_any_tag));
            return comp;
        }
        else if constexpr (tag_type == nbt_type::tag_int_array)
        {
            return impl::any_tag_function::any_tag_get_int_array(std::addressof(impl_any_tag));
        }
        else
        {
            return impl::any_tag_function::any_tag_get_long_array(std::addressof(impl_any_tag));
        }
    }
};

class nbt_document
{
  private:
    impl::nbt_document impl_document;

    template<bool in_place, bool bound_check, std::endian nbt_endian, impl::swapper::read_swapper rswap>
    friend inline  auto read(std::span<std::byte, std::dynamic_extent> source) -> nbt_document;

  public:
    nbt_document(const nbt_document&) = delete;
    nbt_document& operator=(const nbt_document&) = delete;

    inline  nbt_document& operator=(nbt_document&& right) noexcept
    {
        impl::nbt_document_function::nbt_document_move(
            std::addressof(right.impl_document),
            std::addressof(this->impl_document));
        return *this;
    }
    inline  ~nbt_document() noexcept
    {
        impl::nbt_document_function::nbt_document_free(
            std::addressof(this->impl_document));
    }
    inline  nbt_document() noexcept = default;
    inline  nbt_document(nbt_document&& right) noexcept
    {
        impl::nbt_document_function::nbt_document_move(
            std::addressof(right.impl_document),
            std::addressof(this->impl_document));
    }

  public:
    [[nodiscard]] inline  nbt_type type() noexcept
    {
        return static_cast<nbt_type>(impl::swapper::byte_as_type<std::uint8_t>(this->impl_document.source));
    }
    [[nodiscard]] inline  nbt_type element_type() noexcept
    {
        return static_cast<nbt_type>(impl::swapper::byte_as_type<std::uint8_t>(
            this->impl_document.source + sizeof(std::uint8_t) + sizeof(std::uint16_t) +
            impl::swapper::byte_as_type<std::uint16_t>(
                this->impl_document.source + sizeof(std::uint8_t))));
    }
    [[nodiscard]] inline  auto key() noexcept
    {
        return impl::nbt_document_function::nbt_document_root_key(std::addressof(this->impl_document));
    }
    template<nbt_type tag_type, nbt_type list_element_type = nbt_type::tag_end>
    [[nodiscard]] inline  auto value()
    {
        auto real_type{static_cast<nbt_type>(impl::swapper::byte_as_type<std::uint8_t>(this->impl_document.source))};
        if (real_type != tag_type)
        {
            throw nbt_type_error{.is = real_type, .as = tag_type};
        }
        if constexpr (tag_type == nbt_type::tag_list)
        {
            auto list_type{
                static_cast<nbt_type>(impl::swapper::byte_as_type<std::uint8_t>(
                    this->impl_document.source + sizeof(std::uint8_t) + sizeof(std::uint16_t) +
                    impl::swapper::byte_as_type<std::uint16_t>(
                        this->impl_document.source + sizeof(std::uint8_t))))};
            if (list_type != list_element_type)
            {
                throw nbt_type_error{.is = list_type, .as = list_element_type};
            }
        }
        nbt_any_tag<tag_type, list_element_type> tag{};
        tag.impl_any_tag = impl::nbt_document_function::nbt_document_root_value(std::addressof(this->impl_document));
        return tag.get();
    }
};

template<bool in_place = true, bool bound_check = false, std::endian nbt_endian = std::endian::big, impl::swapper::read_swapper rswap = impl::swapper::default_read_swapper>
[[nodiscard]] auto read(std::span<std::byte, std::dynamic_extent> source) -> nbt_document
{
    nbt_document doc{};
    doc.impl_document = impl::read_write::read<in_place, bound_check, nbt_endian, rswap>(source.data(), source.size());
    return doc;
}
}  // namespace na::nbt