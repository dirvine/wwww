# Cross compiling from linux using g++ arm (softfloat)
# On ubuntu, you'll need to install the packages:
# gcc-arm-linux-gnueabi binutils-arm-linux-gnueabi 
#
# Use of this file:
# cmake -DCMAKE_TOOLCHAIN_FILE=../cmake/Toolchain-g++arm-softfloat.cmake ..

# SET(CMAKE_CXX_FLAGS "-m32")
# SET(CMAKE_C_FLAGS "-m32")
# SET(CMAKE_EXE_LINKER_FLAGS "-m32")
# SET(CMAKE_MODULE_LINKER_FLAGS "-m32")
SET(CMAKE_CXX_COMPILER "arm-linux-gnueabi-g++")
SET(CMAKE_C_COMPILER "arm-linux-gnueabi-gcc")
# here is the target environment located
SET(CMAKE_FIND_ROOT_PATH  /usr/arm-linux-gnueabi)

# adjust the default behaviour of the FIND_XXX() commands:
# search headers and libraries in the target environment, search 
# programs in the host environment
set(CMAKE_FIND_ROOT_PATH_MODE_LIBRARY ONLY)
set(CMAKE_FIND_ROOT_PATH_MODE_INCLUDE ONLY)

