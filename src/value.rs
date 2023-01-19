mod value {
    use ndarray::prelude::*;

    #[derive(Clone)]
    struct ValueProps {
        Data: Option<ArrayD<f64>>,
    }

    trait Value {
        fn add<A, B>(&self, a: A, b: B) -> ArrayD<f64>
        where
            A: Into<ArrayD<f64>>,
            B: Into<ArrayD<f64>>;

        fn mult<A, B>(&self, a: A, b: B) -> ArrayD<f64>
        where
            A: Into<ArrayD<f64>>,
            B: Into<ArrayD<f64>>;

        fn print(&self);
    }

    impl Value for ValueProps {
        fn add<A, B>(&self, a: A, b: B) -> ArrayD<f64>
        where
            A: Into<ArrayD<f64>>,
            B: Into<ArrayD<f64>>,
        {
            let a = a.into();
            let b = b.into();
            a + b
        }

        fn mult<A, B>(&self, a: A, b: B) -> ArrayD<f64>
        where
            A: Into<ArrayD<f64>>,
            B: Into<ArrayD<f64>>,
        {
            let a = a.into();
            let b = b.into();
            a * b
        }

        fn print(&self) {
            println!("Value(data={:?})", &self.Data);
        }
    }

    impl ValueProps {
        fn new(data: &ArrayD<f64>) -> Self {
            ValueProps {
                Data: Some(data.clone()),
            }
        }
    }

    #[test]
    fn add() {
        println!("Testing addition");
        let a = ArrayD::ones(vec![2, 1]);
        let b = ArrayD::ones(vec![2, 1]);
        let val = ValueProps::new(&a);
        let result = val.add(a, b);
        assert_eq!(result.dim().as_array_view().dim(), 2);
        assert_eq!(result.shape(), [2, 1]);
    }

    #[test]
    fn mult() {
        println!("Testing multiplication");
        let a = ArrayD::from_shape_vec(vec![4], vec![1.0, 2.0, 3.0, 4.0]).unwrap();
        let b = ArrayD::from_shape_vec(vec![4], vec![1.0, 2.0, 3.0, 4.0]).unwrap();
        let val = ValueProps::new(&a);
        let result = val.mult(a, b);
        println!("{:?}", result);
        assert_eq!(result.dim().as_array_view().dim(), 1);
        assert_eq!(result.shape(), [4]);
    }

    #[test]
    fn print() {
        println!("Testing printing");
        let a = ArrayD::from_shape_vec(vec![4], vec![1.0, 2.0, 3.0, 4.0]).unwrap();
        let val = ValueProps::new(&a);
        val.print();
    }
}
