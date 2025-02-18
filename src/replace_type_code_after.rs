struct Employee {
    employee_type: u8,
    employees_pay_amount: Vec<Box<dyn EmployeeAction>>,
}


impl Employee {
    const ENGINEER: u8 = 0;
    const SALESMAN: u8 = 1;
    const MANAGER: u8 = 2;

    fn new(
        employee_type: u8,
        employees: Vec<Box<dyn EmployeeAction>>,
    ) -> Self {
        Employee { employee_type, employees_pay_amount: employees }
    }

    fn get_employee_type(&self) -> u8 {
        self.employees_pay_amount.iter().map(|e| e.get_employee_type()).sum()
    }

    fn pay_amount(&self) -> u8 {
        self.employees_pay_amount.iter().map(|e| e.pay_amount()).sum()
    }
}


trait EmployeeAction {
    fn get_employee_type(&self) -> u8;
    fn pay_amount(&self) -> u8;
}

struct Engineer {
    name: String,
    salary: u8,
}

impl EmployeeAction for Engineer {
    fn get_employee_type(&self) -> u8 {
        Employee::ENGINEER
    }

    fn pay_amount(&self) -> u8 {
        self.salary
    }
}


struct Salesman {
    name: String,
    sales: u8,
    commission: u8,
}

impl EmployeeAction for Salesman {
    fn get_employee_type(&self) -> u8 {
        Employee::SALESMAN
    }

    fn pay_amount(&self) -> u8 {
        self.sales + self.commission
    }
}

struct Manager {
    name: String,
    salary: u8,
    bonus: u8,
}

impl EmployeeAction for Manager {
    fn get_employee_type(&self) -> u8 {
        Employee::MANAGER
    }

    fn pay_amount(&self) -> u8 {
        self.salary + self.bonus
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_employee() {
        let eng = Engineer { name: "John".to_string(), salary: 100 };
        let emp = Employee::new(Employee::ENGINEER, vec![Box::new(eng)]);
        assert_eq!(emp.get_employee_type(), Employee::ENGINEER);
        assert_eq!(emp.pay_amount(), 100);
    }
}
