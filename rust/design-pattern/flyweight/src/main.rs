use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TreeType {
    pub name: String,
    pub color: String,
    pub texture: String,
}

impl TreeType {
    pub fn new(name: &str, color: &str, texture: &str) -> Self {
        Self {
            name: name.to_string(),
            color: color.to_string(),
            texture: texture.to_string(),
        }
    }

    pub fn draw(&self, canvas: &str, x: i32, y: i32) {
        println!(
            "[{}] にツリーを描画: 位置（{}, {}） - 種類: {}, 色: {}, 質感: {}",
            canvas, x, y, self.name, self.color, self.texture
        );
    }
}

pub struct TreeFactory {
    pool: HashMap<(String, String, String), Rc<TreeType>>,
}

impl TreeFactory {
    pub fn new() -> Self {
        Self {
            pool: HashMap::new(),
        }
    }

    pub fn get_tree_type(&mut self, name: &str, color: &str, texture: &str) -> Rc<TreeType> {
        let key = (name.to_string(), color.to_string(), texture.to_string());

        self.pool
            .entry(key)
            .or_insert_with(|| Rc::new(TreeType::new(name, color, texture)))
            .clone()
    }

    pub fn pool_size(&self) -> usize {
        self.pool.len()
    }
}

pub struct Tree {
    x: i32,
    y: i32,
    tree_type: Rc<TreeType>,
}

impl Tree {
    pub fn new(x: i32, y: i32, tree_type: Rc<TreeType>) -> Self {
        Self { x, y, tree_type }
    }

    pub fn draw(&self, canvas: &str) {
        self.tree_type.draw(canvas, self.x, self.y);
    }
}

pub struct Forest {
    trees: Vec<Tree>,
    factory: TreeFactory,
}

impl Forest {
    pub fn new() -> Self {
        Self {
            trees: Vec::new(),
            factory: TreeFactory::new(),
        }
    }

    pub fn plant_tree(&mut self, x: i32, y: i32, name: &str, color: &str, texture: &str) {
        let tree_type = self.factory.get_tree_type(name, color, texture);
        let tree = Tree::new(x, y, tree_type);
        self.trees.push(tree);
    }

    pub fn draw(&self, canvas: &str) {
        for tree in &self.trees {
            tree.draw(canvas);
        }
    }

    pub fn report(&self) {
        println!("--- 統計レポート ---");
        println!("植えられたツリーの総数: {}", self.trees.len());
        println!(
            "メモリ上に生成された実際の TreeType(フライウェイト)数: {}",
            self.factory.pool_size()
        );
        println!("--------------------\n");
    }
}

fn main() {
    let mut forest = Forest::new();

    forest.plant_tree(10, 20, "東京オーク", "グリーン", "ラフ");
    forest.plant_tree(15, 35, "東京オーク", "グリーン", "ラフ"); // 重複
    forest.plant_tree(50, 70, "東京オーク", "グリーン", "ラフ"); // 重複

    forest.plant_tree(100, 120, "京都メープル", "レッド", "スムース");
    forest.plant_tree(105, 140, "京都メープル", "レッド", "スムース"); // 重複

    forest.plant_tree(200, 250, "北海道パイン", "ダークグリーン", "ソフト");

    forest.report();

    println!("描画を開始します:");
    forest.draw("メインキャンバス");
}
