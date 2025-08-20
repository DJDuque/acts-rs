#include "acts-sys/include/definitions/algebra.hpp"
#include <Eigen/Geometry>

namespace acts_sys {
namespace ffi {

std::unique_ptr<Acts::Transform3>
make_transform3(double rotation_x, double rotation_y, double rotation_z,
                double translation_x, double translation_y,
                double translation_z) {
  Eigen::AngleAxisd rot_x(rotation_x, Eigen::Vector3d::UnitX());
  Eigen::AngleAxisd rot_y(rotation_y, Eigen::Vector3d::UnitY());
  Eigen::AngleAxisd rot_z(rotation_z, Eigen::Vector3d::UnitZ());

  Eigen::Matrix3d rotation;
  rotation = rot_z * rot_y * rot_x;

  Acts::Transform3 tform = Acts::Transform3::Identity();
  tform.linear() = rotation;
  tform.translation() =
      Eigen::Vector3d(translation_x, translation_y, translation_z);

  return std::make_unique<Acts::Transform3>(tform);
}

} // namespace ffi
} // namespace acts_sys
