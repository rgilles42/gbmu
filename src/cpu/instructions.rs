enum ArithmeticTarget{
	reg_A, reg_B, reg_C, reg_D, reg_E, reg_H, reg_L
}

enum Instruction {
	ADD(ArithmeticTarget),
}