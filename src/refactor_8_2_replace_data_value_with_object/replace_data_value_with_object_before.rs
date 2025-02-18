#[derive(Clone)]
struct Order {
     customer: String,
 }

 impl Order {
     fn new(customer: String) -> Order {
         Order {
            customer,
         }
     }

     fn get_customer(&self) -> &str {
         &self.customer
     }

     fn set_customer(&mut self, customer: String) {
         self.customer = customer;
     }
 }

 fn number_of_orders_for(orders: Vec<Order>, customer: &str) -> i32 {
     let mut result = 0;

     orders.iter().for_each(|order| {
         if order.get_customer() == customer {
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