#include "aftermath_libjimage_bind.h"
#include "jimage.hpp"
#include <array>
#include <cstring>
#include <fmt/format.h>
#include <iostream>
#include <map>
#include <vector>

static std::map<std::string, std::vector<char>> classes = {};

bool ctw_visitor(JImageFile *jimage, const char *module_name,
                 const char *version, const char *package, const char *name,
                 const char *extension, void *arg)
{

  if (strcmp(extension, "class") == 0) {

    jlong size;
    JImageLocationRef location = JIMAGE_FindResource(
        jimage, "java.base", "9.0", "java/lang/String.class", &size);
    auto buffer = new char[size];
    auto length = JIMAGE_GetResource(jimage, location, buffer, size);
    std::vector<char> vec_buffer(buffer, buffer + size);
    std::cout << vec_buffer.size() << std::endl;
    auto class_name = fmt::format("{}/{}.{}", package, name, extension);

    classes.insert({class_name, vec_buffer});
  }

  return true;
}

int main(int argc, char **argv)
{
  auto error_code = 0;
  auto value = JIMAGE_Open(
      "/usr/lib/jvm/java-17-openjdk-17.0.3.0.7-2.fc36.x86_64/lib/modules",
      &error_code);
  JIMAGE_ResourceIterator(value, ctw_visitor, nullptr);

  if (error_code != 0) {
    std::cerr << "Failure." << std::endl;
    return 1;
  } else {
    for (const auto [key, value] : classes) {
      std::cout << key << std::endl;

      // std::cout << value.size() << std::endl;
    }
  }

  return 0;
}
