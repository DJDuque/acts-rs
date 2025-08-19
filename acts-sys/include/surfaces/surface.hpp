#pragma once

#include "Acts/Surfaces/Surface.hpp"
#include <memory>

namespace acts_sys {
namespace ffi {

template <typename T, typename... Args>
std::shared_ptr<T> surface_factory(Args... args) {
  return Acts::Surface::makeShared<T>(args...);
}

} // namespace ffi
} // namespace acts_sys
