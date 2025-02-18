struct Employee {
    employee_type: Option<Box<dyn EmployeeTypeAction>>,
    monthly_salary: u32,
    commission: u32,
    bonus: u32,
}

impl Employee {

    fn new( monthly_salary: u32, commission: u32, bonus: u32) -> Employee {
        Employee { employee_type: None, monthly_salary, commission, bonus }
    }

    fn get_employee_type(&self) -> u8 {
        match self.employee_type {
            Some(ref e) => e.get_employee_type(),
            None => panic!("Employee type not set"),
        }
    }

    fn set_employee_type(&mut self, arg: u8) {
        self.employee_type = Option::from(EmployeeType::new(arg));
    }

    fn pay_amount(&self) -> u32 {
        match self.get_employee_type() {
            EmployeeType::ENGINEER => self.monthly_salary ,
            EmployeeType::MANAGER => self.monthly_salary + self.commission,
            EmployeeType::DIRECTOR => self.monthly_salary + self.bonus,
            _ => panic!("Invalid employee type"),
        }
    }
}

struct EmployeeType;

impl EmployeeType {
    const ENGINEER: u8 = 1;
    const MANAGER: u8 = 2;
    const DIRECTOR: u8 = 3;

    fn new(arg: u8) -> Box<dyn EmployeeTypeAction> {
        match arg {
            EmployeeType::ENGINEER =>  Box::new(Engineer),
            EmployeeType::MANAGER =>  Box::new(Manager),
            EmployeeType::DIRECTOR =>  Box::new(Director),
            _ => panic!("Invalid employee type"),
        }
    }
}

trait EmployeeTypeAction {
    fn get_employee_type(&self) -> u8;
}

struct Engineer;

impl EmployeeTypeAction for Engineer {
    fn get_employee_type(&self) -> u8 {
        EmployeeType::ENGINEER
    }
}

struct Manager;
impl EmployeeTypeAction for Manager {
    fn get_employee_type(&self) -> u8 {
        EmployeeType::MANAGER
    }
}

struct Director;
impl EmployeeTypeAction for Director {
    fn get_employee_type(&self) -> u8 {
        EmployeeType::DIRECTOR
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pay_amount() {
        let mut engineer = Employee::new(1000, 0, 0);
        engineer.set_employee_type(EmployeeType::ENGINEER);
        assert_eq!(engineer.pay_amount(), 1000);

        let mut manager = Employee::new(2000, 100, 0);
        manager.set_employee_type(EmployeeType::MANAGER);
        assert_eq!(manager.pay_amount(), 2100);

        let mut director = Employee::new(2000, 0, 1000);
        director.set_employee_type(EmployeeType::DIRECTOR);
        assert_eq!(director.pay_amount(), 3000);

    }
}