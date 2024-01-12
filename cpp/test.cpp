#include "./fast_io/include/fast_io.h"
#include "./fast_io/include/fast_io_device.h"
#include "na_nbt_impl.hpp"
#include <vector>
int main(int argc, char** argv)
{
    {
        fast_io::native_file_loader nfl(fast_io::mnp::os_c_str(argv[1]));
        std::system("pause");
        {
            std::vector<std::byte> data{
                reinterpret_cast<std::byte*>(nfl.begin()),
                reinterpret_cast<std::byte*>(nfl.end())};
            fast_io::io::println(fast_io::out(), "test big_endian read in_place with bound_check");

            using na::nbt::nbt_type;
            std::system("pause");
            auto beforetime = std::chrono::steady_clock::now();
            auto document{na::nbt::read<true, true, std::endian::big>(std::span(data))};
            auto aftertime = std::chrono::steady_clock::now();
            std::system("pause");
            double duration_second = std::chrono::duration<double>(aftertime - beforetime).count();
            auto root = document.value<nbt_type::tag_compound>();

            fast_io::io::println(fast_io::out(), duration_second, "s");
            fast_io::io::println(fast_io::out(), "size@ ", nfl.size(), "bytes");
            fast_io::io::println(fast_io::out(), "speed@ ", nfl.size() / 1000.0 / 1000.0 / duration_second, "mb/s");
            fast_io::io::println(fast_io::out(), "");
        }
        std::system("pause");
    }
}