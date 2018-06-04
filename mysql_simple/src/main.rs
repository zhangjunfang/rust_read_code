#[macro_use]
extern crate mysql;

use mysql as my;

#[derive(Debug, PartialEq, Eq)]
struct Payment {
    customer_id: i32,
    amount: i32,
    account_name: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
struct Myuser {
    user_id: i32,
    name: Option<String>,
    sex: i8,
}

//middle vlue
#[derive(Debug, PartialEq, Eq)]
struct MyuserPayment {
    customer_id: i32,
    amount: i32,
    account_name: Option<String>,
    user_id: i32,
    name: Option<String>,
    sex: i8,
}

fn main() {
    let user = "root";
    let addr = "10.0.4.235";
    let port: u16 = 3306 as u16;
    let pwd: String = String::from("Cme0328@");
    let mut builder = my::OptsBuilder::default();
    builder.user(Some(user)).pass(Some(pwd)).ip_or_hostname(Some(addr)).tcp_port(port).db_name(Some("ocean_01"));
    let pool = my::Pool::new_manual(10, 10, builder).unwrap();
    //
    pool.prep_exec(r"CREATE TABLE IF NOT EXISTS payment (
                          customer_id int not null,
                          amount int not null,
                          account_name text
                      )", ()).unwrap();
    pool.prep_exec(r"CREATE TABLE IF NOT EXISTS myuser (
                          user_id int not null,
                          name varchar(20) not null,
                          sex char(1)
                      )", ()).unwrap();
    pool.prep_exec(r"delete from  payment ", ()).unwrap();
    pool.prep_exec(r"delete from  myuser ", ()).unwrap();
    let payments = vec![
        Payment { customer_id: 1, amount: 2, account_name: Some("zhangboyu".into()) },
        Payment { customer_id: 3, amount: 4, account_name: Some("foo".into()) },
        Payment { customer_id: 5, amount: 6, account_name: Some("food".into()) },
        Payment { customer_id: 7, amount: 8, account_name: None },
        Payment { customer_id: 9, amount: 10, account_name: Some("bar".into()) },
    ];


    let myuser = vec![
        Myuser { user_id: 1i32, name: Some("zhangboyu01".into()), sex: 1i8 },
        Myuser { user_id: 3i32, name: Some("zhangboyu02".into()), sex: 2i8 },
        Myuser { user_id: 5i32, name: Some("zhangboyu03".into()), sex: 3i8 },
        Myuser { user_id: 7i32, name: Some("zhangboyu04".into()), sex: 4i8 },
        Myuser { user_id: 9i32, name: Some("zhangboyu05".into()), sex: 5i8 },
    ];

    for mut stmt in pool.prepare(r"INSERT INTO myuser
                                        (user_id, name, sex)
                                    VALUES
                                        (:user_id, :name, :sex)").into_iter() {
        for m in myuser.iter() {
            stmt.execute(params! {
                 "user_id" => m.user_id,
                 "name" => m.name,
                 "sex" => m.sex,
            }).unwrap();
        }
    }


    for mut stmt in pool.prepare(r"INSERT INTO payment
                                        (customer_id, amount, account_name)
                                    VALUES
                                        (:customer_id, :amount, :account_name)").into_iter() {
        for p in payments.iter() {
            stmt.execute(params! {
                 "customer_id" => p.customer_id,
                 "amount" => p.amount,
                 "account_name" => &p.account_name,
             }).unwrap();
        }
    }
    for mut stmt in pool.prepare(r"INSERT INTO myuser
                                        (user_id, name, sex)
                                    VALUES
                                        (:user_id, :name, :sex)").into_iter() {
        stmt.execute(params! {
                 "user_id" => 1i32,
                 "name" => String::from("11"),
                 "sex" => {1i8},
             }).unwrap();
    }
    for mut stmt in pool.prepare(r"update myuser set myuser.name=:name  where myuser.user_id=:id").into_iter() {
        stmt.execute(params! {
                 "name" => String::from("1111"),
                 "id" => 1i32,
             }).unwrap();
    }

    let selected_payments: Vec<MyuserPayment> =
        pool.prep_exec("SELECT myuser.user_id,myuser.name,myuser.sex,payment.amount,payment.account_name from myuser ,payment where myuser.user_id=payment.customer_id", ())
            .map(|result| {
                result.map(|x| x.unwrap()).map(|row| {
                    let (user_id, name, sex, amount, account_name) = my::from_row(row);
                    MyuserPayment {
                        user_id: user_id,
                        name: name,
                        sex: sex,
                        customer_id: user_id,
                        amount: amount,
                        account_name: account_name,
                    }
                }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Payment>`
            }).unwrap(); // Unwrap `Vec<Payment>`
    for my in &selected_payments {
        println!("{:#?}", my);
    }

    println!("Yay!");
}