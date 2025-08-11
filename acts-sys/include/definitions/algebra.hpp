#pragma once

#include "Acts/Definitions/Algebra.hpp"

namespace acts_sys {
namespace ffi {

	inline double transform_equivalent_tolerance() {
		return Acts::s_transformEquivalentTolerance;
	}

}
}
