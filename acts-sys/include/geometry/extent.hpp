#pragma once

#include "Acts/Geometry/Extent.hpp"
#include <memory>

namespace acts_sys {
namespace ffi {

inline std::unique_ptr<Acts::ExtentEnvelope> new_extent_envelope(
    const std::array<double, 2> &x, const std::array<double, 2> &y,
    const std::array<double, 2> &z, const std::array<double, 2> &r,
    const std::array<double, 2> &phi, const std::array<double, 2> &r_phi,
    const std::array<double, 2> &theta, const std::array<double, 2> &eta,
    const std::array<double, 2> &mag) {
  Acts::ExtentEnvelope::Arguments args{.x = x,
                                       .y = y,
                                       .z = z,
                                       .r = r,
                                       .phi = phi,
                                       .rPhi = r_phi,
                                       .theta = theta,
                                       .eta = eta,
                                       .mag = mag};

  return std::make_unique<Acts::ExtentEnvelope>(std::move(args));
}

} // namespace ffi
} // namespace acts_sys
