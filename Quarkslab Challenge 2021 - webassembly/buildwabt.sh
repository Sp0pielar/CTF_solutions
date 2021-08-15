set -e #stop script if any cmmand returns non 0

mkdir -p wabt/build
cd wabt/build
cmake -DBUILD_TESTS=OFF -DCMAKE_BUILD_TYPE=debug ..
cmake --build .

