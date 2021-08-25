#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Operation {
    Operator(char),
    Add,
    Subt,
    Mult,
    Div,
    Mod,
    Pow,
    Invalid,
}

impl Operation {
    pub fn from_operator(&self) -> Operation {
        if let Operation::Operator(val) = self {
            match val {
                '+' => Operation::Add,
                '-' => Operation::Subt,
                '*' => Operation::Mult,
                '/' => Operation::Div,
                '%' => Operation::Mod,
                '^' => Operation::Pow,
                _ => Operation::Invalid,
            }
        } else {
            self.clone()
        }
    }

    pub fn order(&self) -> OperationOrder {
        match self {
            &Self::Add | &Self::Subt => OperationOrder::Second,
            &Self::Mult | &Self::Div | &Self::Mod => OperationOrder::First,
            &Self::Pow => OperationOrder::Power,
            _ => OperationOrder::None,
        }
    }
}

#[derive(Eq, PartialEq)]
pub enum OperationOrder {
    Power,
    First,  // Mult, Div, Mod
    Second, // Add, Subt
    None,
}
