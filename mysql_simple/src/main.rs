#[macro_use]
extern  crate mysql;
use mysql  as my;



#[derive(Debug, PartialEq, Eq)]
struct Payment {
    customer_id: i32,
    amount: i32,
    account_name: Option<String>,
}





 fn main() {
   let user = "root";
   let addr = "127.0.0.1";
   let port: u16 = std::env::var("MYSQL_SERVER_PORT").ok()
                              .map(|my_port|
                                  my_port.parse::<u16>().ok().unwrap_or(3307)
                              ).unwrap_or(3307);
   let pwd: String = std::env::var("MYSQL_SERVER_PASS").unwrap_or("password".to_string());
   let pool = if port == 3307 && pwd == "password" {
     let pool = my::Pool::new("mysql://root:password@localhost:3307").unwrap();
       drop(pool);
       my::Pool::new_manual(1, 1, "mysql://root:password@localhost:3307").unwrap()
   } else {
       let mut builder = my::OptsBuilder::default();
       builder.user(Some(user))
              .pass(Some(pwd))
              .ip_or_hostname(Some(addr))
              .tcp_port(port);
       my::Pool::new_manual(1, 1, builder).unwrap()
   };

     // Let's create payment table.
     // It is temporary so we do not need `tmp` database to exist.
     // Unwap just to make sure no error happened.
     pool.prep_exec(r"CREATE TEMPORARY TABLE tmp.payment (
                          customer_id int not null,
                          amount int not null,
                          account_name text
                      )", ()).unwrap();

     let payments = vec![
         Payment { customer_id: 1, amount: 2, account_name: None },
         Payment { customer_id: 3, amount: 4, account_name: Some("foo".into()) },
         Payment { customer_id: 5, amount: 6, account_name: None },
         Payment { customer_id: 7, amount: 8, account_name: None },
         Payment { customer_id: 9, amount: 10, account_name: Some("bar".into()) },
     ];

     // Let's insert payments to the database
     // We will use into_iter() because we do not need to map Stmt to anything else.
     // Also we assume that no error happened in `prepare`.
     for mut stmt in pool.prepare(r"INSERT INTO tmp.payment
                                        (customer_id, amount, account_name)
                                    VALUES
                                        (:customer_id, :amount, :account_name)").into_iter() {
         for p in payments.iter() {
             // `execute` takes ownership of `params` so we pass account name by reference.
             // Unwrap each result just to make sure no errors happened.
             stmt.execute(params!{
                 "customer_id" => p.customer_id,
                 "amount" => p.amount,
                 "account_name" => &p.account_name,
             }).unwrap();
         }
     }

     // Let's select payments from database
     let selected_payments: Vec<Payment> =
     pool.prep_exec("SELECT customer_id, amount, account_name from tmp.payment", ())
     .map(|result| { // In this closure we will map `QueryResult` to `Vec<Payment>`
         // `QueryResult` is iterator over `MyResult<row, err>` so first call to `map`
         // will map each `MyResult` to contained `row` (no proper error handling)
         // and second call to `map` will map each `row` to `Payment`
         result.map(|x| x.unwrap()).map(|row| {
             let (customer_id, amount, account_name) = my::from_row(row);
             Payment {
                 customer_id: customer_id,
                 amount: amount,
                 account_name: account_name,
             }
         }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Payment>`
     }).unwrap(); // Unwrap `Vec<Payment>`

     // Now make sure that `payments` equals to `selected_payments`.
     // Mysql gives no guaranties on order of returned rows without `ORDER BY`
     // so assume we are lukky.
     assert_eq!(payments, selected_payments);
     println!("Yay!");
 }

