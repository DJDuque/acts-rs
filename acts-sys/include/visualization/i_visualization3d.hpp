#pragma once

#include "Acts/Visualization/IVisualization3D.hpp"
#include <filesystem>

namespace acts_sys {
namespace ffi {

inline void i_visualization3d_write(const Acts::IVisualization3D &vis,
                                    const std::string &path) {
  std::filesystem::path fs_path = path;
  vis.write(fs_path);
}

} // namespace ffi
} // namespace acts_sys
