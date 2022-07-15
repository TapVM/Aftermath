#include "../include/aftermath_libjimage_bind.h"
#include "../include/jimage.hpp"
#include <cstring>
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
    auto location =
        JIMAGE_FindResource(jimage, module_name, version, name, &size);
    auto buffer = std::vector<char>();
    JIMAGE_GetResource(jimage, location, buffer.data(), size);
    classes.insert(std::string(package), buffer);
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
  return 0;
}
