#pragma once

#include <memory>

namespace acts_sys {
namespace ffi {

#define FFI_CONSTANT_FUNCTION(namespace, constant)                             \
  inline auto constant() { return namespace ::constant; }

// See https://github.com/dtolnay/cxx/issues/280 and check if the status around
// constructors has changed.
template <typename T, typename... Args>
std::unique_ptr<T> make_unique(Args... args) {
  return std::make_unique<T>(args...);
}

template <typename Base, typename Derived>
inline Base &upcast(Derived &derived) {
  return derived;
}

template <typename Base, typename Derived>
inline const Base &upcast(const Derived &derived) {
  return derived;
}

template <typename Base, typename Derived>
inline std::unique_ptr<Base> upcast(std::unique_ptr<Derived> derived) {
  return std::unique_ptr<Base>(std::move(derived));
}

} // namespace ffi
} // namespace acts_sys
