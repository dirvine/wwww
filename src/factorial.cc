#include <iostream>
#include "boost/mp_math/mp_int.hpp"

typedef boost::mp_math::mp_int<> BigInt;

BigInt factorial(BigInt n) {
	if (n<=1){
		return n;
	} else {
		return(n * factorial(n - 1));
	}
}

int main(int argc, char **argv){
	BigInt n ;
	std::cout << "input number " << std::endl;
	std::cin >> n ;
	std::cout << factorial(n) <<std::endl;
}

