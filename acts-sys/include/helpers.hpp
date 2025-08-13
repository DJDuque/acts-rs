#pragma once

namespace acts_sys {
namespace ffi {

#define FFI_CONSTANT_FUNCTION(namespace, constant)                             \
  inline auto constant() { return namespace ::constant; }

} // namespace ffi
} // namespace acts_sys
