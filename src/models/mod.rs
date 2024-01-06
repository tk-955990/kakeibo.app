use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

// 収入カテゴリ
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum IncomeCategory {
    Salary,
    Bonus,
    Other,
}
// 支出カテゴリ
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum ExpenseCategory {
    Food,
    Hobby,
    Other,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Category {
    Income(IncomeCategory),
    Expense(ExpenseCategory),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Item {
    name: String,
    category: Category,
    price: u32,
    date: NaiveDate,
}

impl Item {
    pub fn new(name: String, category: Category, price: u32, date: NaiveDate) -> Self {
        Item {
            name,
            category,
            price,
            date,
        }
    }

    pub fn get_category(register_type: u8, category_type: u8) -> Category {
        if register_type == 0 {
            match category_type {
                0 => Category::Income(IncomeCategory::Salary),
                1 => Category::Income(IncomeCategory::Bonus),
                2 => Category::Income(IncomeCategory::Other),
                _ => panic!("不正なカテゴリ種別です"),
            }
        } else {
            match category_type {
                0 => Category::Expense(ExpenseCategory::Food),
                1 => Category::Expense(ExpenseCategory::Hobby),
                2 => Category::Expense(ExpenseCategory::Other),
                _ => panic!("不正なカテゴリ種別です"),
            }
        }
    }

    // 集計用関数
    pub fn get_year(&self) -> i32 {
        self.date.year()
    }

    pub fn get_month(&self) -> u32 {
        self.date.month()
    }

    // from_ymd(非推奨)
    // pub fn get_first_day(&self) -> NaiveDate {
    //     NaiveDate::from_ymd(self.get_year(), self.get_month(), 1)
    // }

    // 月ごとの集計のため、日付を1日に変換
    pub fn get_first_day(&self) -> NaiveDate {
        let year = self.get_year();
        let month = self.get_month();
        let day = 1;
        NaiveDate::from_ymd_opt(year, month, day).expect("Invalid date")
    }

    // 収入と支出(支出はマイナス表記のため i32 に変換)
    pub fn get_price_for_summary(&self) -> i32 {
        match self.category {
            Category::Income(_) => self.price as i32,
            Category::Expense(_) => -1 * self.price as i32,
        }
    }
}
