//-------------- libjimage_bind - Instruction class definition ---------------//
//
// Part of the Aftermath JVM, under the MIT license. The LICENSE file is present
// at the project root, please consult to it for license information.
//
//===----------------------------------------------------------------------===//
//
// This file is a thin wrapper for libjimage, providing Rust-friendly bindings.
// It depends on
// - libfmt (vendored in the `libfmt` directory of aftermath_libjimage_bind).
//
//===----------------------------------------------------------------------===//

#include <aftermath_libjimage_bind.h> /* Exposing library methods             */
#include <cstring>                    /* strcmp()                             */
#include <fmt/format.h>               /* libfmt                               */
#include <iostream>                   /* Logging                              */
#include <jimage.hpp>                 /* Bindings to libjimage                */
#include <map>                        /* std::map                             */
#include <vector>                     /* std::vector                          */

// SAFETY -> `classes` will *never* be mutated concurrently/in parallel.
// All operations to classes will happen sequentially.
static std::map<std::string, std::vector<char>> classes = {};

bool ctw_visitor(JImageFile *jimage, const char *_module_name,
                 const char *_version, const char *package, const char *name,
                 const char *extension, void *_arg)
{
  if (strcmp(extension, "class") == 0) {
    jlong size;
    auto class_name = fmt::format("{}/{}.{}", package, name, extension);
    JImageLocationRef location = JIMAGE_FindResource(
        jimage, package, "9.0", "java/lang/String.class", &size);

    char *buffer = new char[size];
    JIMAGE_GetResource(jimage, location, buffer, size);
    std::vector<char> vec_buffer(buffer, buffer + size);

    classes.insert({class_name, vec_buffer});
  }

  return true;
}

int main()
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
    for (const auto &[key, value] : classes) {
      std::cout << key << std::endl;

      // std::cout << value.size() << std::endl;
    }
  }

  return 0;
}
