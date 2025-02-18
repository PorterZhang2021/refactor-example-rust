struct Employee {
    employee_type: Box<dyn EmployeeType>,
    monthly_salary: u32,
    commission: u32,
    bonus: u32,
}

impl Employee {
    const ENGINEER: u8 = 1;
    const MANAGER: u8 = 2;
    const DIRECTOR: u8 = 3;
    fn new(employee_type: Box<dyn EmployeeType>, monthly_salary: u32, commission: u32, bonus: u32) -> Employee {
        Employee { employee_type, monthly_salary, commission, bonus }
    }

    fn get_employee_type(&self) -> u8 {
        self.employee_type.get_employee_type()
    }
    fn pay_amount(&self) -> u32 {
        match self.get_employee_type() {
            Employee::ENGINEER => self.monthly_salary ,
            Employee::MANAGER => self.monthly_salary + self.commission,
            Employee::DIRECTOR => self.monthly_salary + self.bonus,
            _ => panic!("Invalid employee type"),
        }
    }
}

trait EmployeeType {
    fn get_employee_type(&self) -> u8;
}

struct Engineer;

impl EmployeeType for Engineer {
    fn get_employee_type(&self) -> u8 {
        Employee::ENGINEER
    }
}

struct Manager;
impl EmployeeType for Manager {
    fn get_employee_type(&self) -> u8 {
        Employee::MANAGER
    }
}

struct Director;
impl EmployeeType for Director {
    fn get_employee_type(&self) -> u8 {
        Employee::DIRECTOR
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pay_amount() {
        let engineer = Employee::new(Box::new(Engineer), 1000, 0, 0);
        assert_eq!(engineer.pay_amount(), 1000);

        let manager = Employee::new(Box::new(Manager), 2000, 100, 0);
        assert_eq!(manager.pay_amount(), 2100);

        let director = Employee::new(Box::new(Director), 2000, 0, 1000);
        assert_eq!(director.pay_amount(), 3000);

    }
}