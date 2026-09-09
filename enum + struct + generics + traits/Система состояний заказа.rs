use std::{vec, println};

#[derive(Debug, Clone, PartialEq)]
enum OrderStatus {
    Pending,
    Processing { assigned_worker: String },
    Shipped { tracking_number: String },
    Delivered { delivered_at: String },
    Cancelled { reason: String },
}

trait StatusProcessor {
    fn can_transition_to(&self, new_status: &OrderStatus) -> bool;
    fn next_possible_states(&self) -> Vec<OrderStatus>;
    fn is_final_state(&self) -> bool;
}

// Реализуйте StatusProcessor для OrderStatus
// Правила:
// - Pending может перейти в Processing или Cancelled
// - Processing может перейти в Shipped или Cancelled  
// - Shipped может перейти в Delivered
// - Delivered и Cancelled - финальные состояния

impl StatusProcessor for OrderStatus {
    // Ваша реализация
    fn can_transition_to(&self, new_status: &OrderStatus) -> bool {
        match self {
            OrderStatus::Pending => self.next_possible_states().contains(new_status),
            OrderStatus::Processing { assigned_worker: _ } => self.next_possible_states().contains(new_status),
            OrderStatus::Shipped { tracking_number: _ } => self.next_possible_states().contains(new_status),
            OrderStatus::Delivered { delivered_at: _ } => self.next_possible_states().contains(new_status),
            OrderStatus::Cancelled { reason: _ } => self.next_possible_states().contains(new_status),
        }
    }

    fn next_possible_states(&self) -> Vec<OrderStatus> {
        match self {
            OrderStatus::Pending => vec![
                OrderStatus::Processing { assigned_worker: "Name".to_string() },
                OrderStatus::Cancelled { reason: "Error".to_string() },
            ],
            OrderStatus::Processing { assigned_worker: _ } => vec![
                OrderStatus::Shipped { tracking_number: "Name".to_string() },
                OrderStatus::Cancelled { reason: "Name".to_string() },
            ],
            OrderStatus::Shipped { tracking_number: _ } => vec![
                OrderStatus::Delivered { delivered_at: "Mike".to_string() },
            ],
            _ => Vec::new(),
        }
    }

    fn is_final_state(&self) -> bool {
        match self {
            OrderStatus::Delivered { delivered_at: _ } => true,
            OrderStatus::Cancelled { reason: _ } => true,
            _ => false,
        }
    }
}

// Создайте функцию для обработки перехода состояний
fn transition_order(current: OrderStatus, new: OrderStatus) -> Result<OrderStatus, String> {
    // Проверить возможность перехода и вернуть новое состояние
    if current.is_final_state() {
        return Err("Заказ на финальной стадии!".to_string());
    }

    if current.can_transition_to(&new) {
        Ok(new)
    } else {
        Err("Ошибка в переходе между состояниями заказов".to_string())
    }
}

fn main() {
    let order = OrderStatus::Pending;

    println!("{:?}", transition_order(OrderStatus::Delivered { delivered_at: "Name".to_string() }, OrderStatus::Pending ));
}