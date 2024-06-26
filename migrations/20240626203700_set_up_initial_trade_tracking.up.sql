-- Add up migration script here
create table trades (
    -- Trade info
    id serial primary key,
    symbol varchar(20) not null,
    interval varchar(20) not null,
    orientation varchar(10) not null,
    trading_strategy varchar(50) not null,
    resolution_strategy varchar(50) not null,
    data_source varchar(50) not null,  

    -- Time info
    entered_at timestamp not null, 
    exited_at timestamp,
    bars_in_trade integer,

    -- Numerical info
    entry_price numeric(20,10) not null,
    exit_price numeric(20,10),
    quantity numeric(20,10) not null,
    dollar_value numeric(20,2) not null,
    entry_fee numeric(20,10) default 0,
    exit_fee numeric(20,10) default 0,

    -- misc
    comments text,

    -- calculated field 
    profit_loss numeric(20,2) generated always as (
        exit_price * quantity - entry_price * quantity - entry_fee - exit_fee
    ) stored,
    profit_loss_pct numeric(10,2) generated always as (
        case 
            when entry_price = 0 then 0
            else ((exit_price - entry_price) / entry_price) * 100
        end
    ) stored,
    is_completed boolean generated always as (
        exit_price is not null
    ) stored
);

CREATE INDEX idx_symbol ON trades(symbol);
CREATE INDEX idx_entered_at ON trades(entered_at);
CREATE INDEX idx_exited_at ON trades(exited_at);

