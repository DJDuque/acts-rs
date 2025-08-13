#pragma once

#include "Acts/Definitions/Tolerance.hpp"
#include "acts-sys/include/helpers.hpp"

namespace acts_sys {
namespace ffi {

FFI_CONSTANT_FUNCTION(Acts, s_epsilon)
FFI_CONSTANT_FUNCTION(Acts, s_onSurfaceTolerance)
FFI_CONSTANT_FUNCTION(Acts, s_curvilinearProjTolerance)

} // namespace ffi
} // namespace acts_sys
