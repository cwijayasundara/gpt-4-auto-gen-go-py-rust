from sqlalchemy.orm import Session
from models import Trade
from schemas import TradeCreate


def create_trade(db: Session, trade: TradeCreate):
    db_trade = Trade(**trade.dict())
    db.add(db_trade)
    db.commit()
    db.refresh(db_trade)
    return db_trade


def get_trades(db: Session):
    return db.query(Trade).all()


def get_trade(db: Session, trade_id: int):
    return db.query(Trade).filter(Trade.id == trade_id).first()


def update_trade(db: Session, trade_id: int, trade: TradeCreate):
    db_trade = db.query(Trade).filter(Trade.id == trade_id).first()
    db_trade.value = trade.value
    db_trade.date = trade.date
    db_trade.trader = trade.trader
    db.commit()
    return db_trade


def delete_trade(db: Session, trade_id: int):
    db_trade = db.query(Trade).filter(Trade.id == trade_id).first()
    db.delete(db_trade)
    db.commit()
