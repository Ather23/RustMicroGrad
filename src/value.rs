mod value {
    use ndarray::prelude::*;

    #[derive(Clone, Debug)]
    struct Value {
        data: Option<ArrayD<f64>>,
        prev_ops: Option<Vec<PrevOp>>,
    }

    #[derive(Clone, Debug)]
    struct PrevOp {
        op: Value,
    }

    trait IValue {
        fn add(&self, b: Value) -> Value;
        fn mult(&self, b: Value) -> Value;
        fn prev_ops(&self, b: Value) -> Vec<PrevOp>;
        fn print(&self);
    }

    impl IValue for Value {
        fn add(&self, b: Value) -> Value {
            let add = &self.data.clone().unwrap() + b.data.clone().unwrap();
            let mut out = Value::new(&add);
            out.prev_ops = Some(self.prev_ops(b));
            return out;
        }

        fn mult(&self, b: Value) -> Value {
            let mul = &self.data.clone().unwrap() * b.data.clone().unwrap();
            let mut out = Value::new(&mul);
            out.prev_ops = Some(self.prev_ops(b));
            return out;
        }

        fn prev_ops(&self, b: Value) -> Vec<PrevOp> {
            let mut prev_set: Vec<PrevOp> = Vec::new();
            prev_set.push(PrevOp { op: self.clone() });
            prev_set.push(PrevOp { op: b });
            prev_set
        }

        fn print(&self) {
            println!("Value(data={:?})", &self.data);
        }
    }

    impl Value {
        fn new(data: &ArrayD<f64>) -> Self {
            Value {
                data: Some(data.clone()),
                prev_ops: None,
            }
        }
    }

    #[test]
    fn add() {
        println!("Testing addition");
        let a = ArrayD::ones(vec![2, 1]);
        let b = ArrayD::ones(vec![2, 1]);

        let val_a = Value::new(&a);
        let val_b = Value::new(&b);
        let result = val_a.add(val_b);
        result.print();

        println!("Previous ops {:?}", result.prev_ops);
    }

    #[test]
    fn mult() {
        println!("Testing multiplication");
        let a = ArrayD::from_shape_vec(vec![4], vec![1.0, 2.0, 3.0, 4.0]).unwrap();
        let b = ArrayD::from_shape_vec(vec![4], vec![1.0, 2.0, 3.0, 4.0]).unwrap();

        let val_a = Value::new(&a);
        let val_b = Value::new(&b);
        let result = val_a.mult(val_b);
        println!("{:?}", result);
    }
    #[test]
    fn computation() {
        println!("Testing multiplication");
        let a = ArrayD::from_shape_vec(vec![1], vec![2.0]).unwrap();
        let b = ArrayD::from_shape_vec(vec![1], vec![-3.0]).unwrap();
        let c = ArrayD::from_shape_vec(vec![1], vec![10.0]).unwrap();

        let val_a = Value::new(&a);
        let val_b = Value::new(&b);
        let val_c = Value::new(&c);
        let result = val_a.mult(val_b).add(val_c);

        println!("Previous ops {:?}", result.prev_ops);
        let prev = result.prev_ops.unwrap()[0].op.to_owned().data.unwrap();
        print!("{:?}", prev);
        assert_eq!(prev[0], -6.)
    }

    #[test]
    fn print() {
        println!("Testing printing");
        let a = ArrayD::from_shape_vec(vec![4], vec![1.0, 2.0, 3.0, 4.0]).unwrap();
        let val = Value::new(&a);
        val.print();
    }
}
