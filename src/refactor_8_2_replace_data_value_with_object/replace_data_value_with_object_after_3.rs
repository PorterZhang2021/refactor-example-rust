
#[derive(Clone)]
struct Customer {
    name: String,
}

impl Customer {
    fn new(name: String) -> Customer {
        Customer {
            name,
        }
    }
    fn get_name(&self) -> &str {
        &self.name
    }
}

#[derive(Clone)]
 struct Order {
     customer: Customer,
 }


 impl Order {
     fn new(customer_name: String) -> Order {
         Order {
             customer: Customer::new(customer_name),
         }
     }

     fn get_customer_name(&self) -> &str {
         self.customer.get_name()
     }

     fn set_customer(&mut self, customer_name: String) {
         self.customer = Customer::new(customer_name);
     }
 }

 fn number_of_orders_for(orders: Vec<Order>, customer: &str) -> i32 {
     let mut result = 0;

     orders.iter().for_each(|order| {
         if order.get_customer_name() == customer {
             result += 1;
         }
     });

     result
 }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_orders() {
        let orders = vec![Order::new("Alice".to_string()), Order::new("Bob".to_string())];
        assert_eq!(1, number_of_orders_for(orders.clone(), "Alice"));
        assert_eq!(0, number_of_orders_for(orders.clone(), "Charlie"));
    }
}