#pragma once

#include "Acts/Definitions/Algebra.hpp"
#include "acts-sys/include/helpers.hpp"
#include <memory>

namespace acts_sys {
namespace ffi {

FFI_CONSTANT_FUNCTION(Acts, s_transformEquivalentTolerance)

std::unique_ptr<Acts::Transform3>
make_transform3(double rotation_x, double rotation_y, double rotation_z,
                double translation_x, double translation_y,
                double translation_z);

} // namespace ffi
} // namespace acts_sys
