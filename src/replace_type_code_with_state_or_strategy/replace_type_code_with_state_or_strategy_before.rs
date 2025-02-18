struct Employee {
    employee_type: u8,
    monthly_salary: u32,
    commission: u32,
    bonus: u32,
}

impl Employee {
    const ENGINEER: u8 = 1;
    const MANAGER: u8 = 2;
    const DIRECTOR: u8 = 3;
    fn new(employee_type: u8, monthly_salary: u32, commission: u32, bonus: u32) -> Employee {
        Employee { employee_type, monthly_salary, commission, bonus }
    }

    fn pay_amount(&self) -> u32 {
        match self.employee_type {
            Employee::ENGINEER => self.monthly_salary ,
            Employee::MANAGER => self.monthly_salary + self.commission,
            Employee::DIRECTOR => self.monthly_salary + self.bonus,
            _ => panic!("Invalid employee type"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pay_amount() {
        let engineer = Employee::new(Employee::ENGINEER, 1000, 0, 0);
        assert_eq!(engineer.pay_amount(), 1000);

        let manager = Employee::new(Employee::MANAGER, 2000, 100, 0);
        assert_eq!(manager.pay_amount(), 2100);

        let director = Employee::new(Employee::DIRECTOR, 2000, 0, 1000);
        assert_eq!(director.pay_amount(), 3000);

    }
}