-- Таблица заказов (orders)
CREATE TABLE orders (
    order_id SERIAL PRIMARY KEY,
    order_uid VARCHAR(50) NOT NULL UNIQUE,
    track_number VARCHAR(50) NOT NULL,
    entry VARCHAR(50) NOT NULL,
    locale VARCHAR(10) NOT NULL,
    customer_id VARCHAR(50) NOT NULL,
    delivery_service VARCHAR(50) NOT NULL,
    shardkey VARCHAR(10) NOT NULL,
    sm_id INT NOT NULL,
    oof_shard VARCHAR(10) NOT NULL
);

-- Таблица платежей (payments), ссылается на таблицу orders
CREATE TABLE payments (
    payment_id SERIAL PRIMARY KEY,
    order_id INT NOT NULL,
    transaction VARCHAR(50) NOT NULL,
    request_id VARCHAR(50) NOT NULL,
    currency VARCHAR(10) NOT NULL,
    provider VARCHAR(50) NOT NULL,
    amount DECIMAL(10, 2) NOT NULL,
    bank VARCHAR(50) NOT NULL,
    delivery_cost DECIMAL(10, 2) NOT NULL,
    goods_total DECIMAL(10, 2) NOT NULL,
    custom_fee DECIMAL(10, 2) NOT NULL,
    FOREIGN KEY (order_id) REFERENCES orders(order_id)
);

-- Таблица товаров (items), ссылается на таблицу orders
CREATE TABLE items (
    item_id SERIAL PRIMARY KEY,
    order_id INT NOT NULL,
    chrt_id INT NOT NULL,
    track_number VARCHAR(50) NOT NULL,
    price DECIMAL(10, 2) NOT NULL,
    rid VARCHAR(50) NOT NULL,
    name VARCHAR(100) NOT NULL,
    sale DECIMAL(5, 2) NOT NULL,
    size VARCHAR(10) NOT NULL,
    total_price DECIMAL(10, 2) NOT NULL,
    nm_id INT NOT NULL,
    brand VARCHAR(50) NOT NULL,
    status INT NOT NULL,
    FOREIGN KEY (order_id) REFERENCES orders(order_id)
);

-- Таблица доставки (delivery), ссылается на таблицу orders
CREATE TABLE delivery (
    delivery_id SERIAL PRIMARY KEY,
    order_id INT NOT NULL,
    name VARCHAR(100) NOT NULL,
    phone VARCHAR(20) NOT NULL,
    zip VARCHAR(10) NOT NULL,
    city VARCHAR(50) NOT NULL,
    address VARCHAR(100) NOT NULL,
    region VARCHAR(50) NOT NULL,
    email VARCHAR(100) NOT NULL,
    FOREIGN KEY (order_id) REFERENCES orders(order_id)
);