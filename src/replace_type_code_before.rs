struct Employee {
    employee_type: u8,
    monthly_salary: u8,
    commission: u8,
    bonus: u8,
}


impl Employee {
    const ENGINEER: u8 = 0;
    const SALESMAN: u8 = 1;
    const MANAGER: u8 = 2;

    fn new(
        employee_type: u8,
        monthly_salary: u8,
        commission: u8,
        bonus: u8,
    ) -> Self {
        Employee { employee_type, monthly_salary, commission, bonus }
    }

    fn get_employee_type(&self) -> u8 {
        self.employee_type
    }

    fn pay_amount(&self) -> u8 {
        match self.employee_type {
            Self::ENGINEER => self.monthly_salary,
            Self::SALESMAN => self.monthly_salary + self.commission,
            Self::MANAGER => self.monthly_salary + self.bonus,
            _ => panic!("Invalid employee type"),
        }
    }

}
