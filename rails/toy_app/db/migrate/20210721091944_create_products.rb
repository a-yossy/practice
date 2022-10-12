class CreateProducts < ActiveRecord::Migration[5.1]
  def change
    create_table :products do |t|
      t.string :name, null: false
      t.integer :price, null: false
      t.string :unit, null: false
      t.boolean :availability, null: false

      t.timestamps
    end
  end
end
