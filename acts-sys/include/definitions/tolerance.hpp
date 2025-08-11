#pragma once

#include "Acts/Definitions/Tolerance.hpp"

namespace acts_sys {
namespace ffi {

inline double epsilon() { return Acts::s_epsilon; }

inline double on_surface_tolerance() { return Acts::s_onSurfaceTolerance; }

inline double curvilinear_proj_tolerance() {
  return Acts::s_curvilinearProjTolerance;
}

} // namespace ffi
} // namespace acts_sys
