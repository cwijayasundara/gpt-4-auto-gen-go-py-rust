from fastapi import FastAPI, HTTPException, Depends
from sqlalchemy.orm import Session
from database import SessionLocal, engine
import models
from schemas import Trade, TradeCreate
import crud
from typing import List

models.Base.metadata.create_all(bind=engine)

app = FastAPI()


def get_db():
    db = SessionLocal()
    try:
        yield db
    finally:
        db.close()


@app.post("/trades/", response_model=Trade)
def create_trade(trade: TradeCreate, db: Session = Depends(get_db)):
    return crud.create_trade(db=db, trade=trade)


@app.get("/trades/", response_model=List[Trade])
def read_trades(skip: int = 0, limit: int = 100, db: Session = Depends(get_db)):
    trades = crud.get_trades(db)
    return trades


@app.get("/trades/{trade_id}", response_model=Trade)
def read_trade(trade_id: int, db: Session = Depends(get_db)):
    trade = crud.get_trade(db, trade_id=trade_id)
    if trade is None:
        raise HTTPException(status_code=404, detail="Trade not found")
    return trade


@app.put("/trades/{trade_id}", response_model=Trade)
def update_trade(trade_id: int, trade: TradeCreate, db: Session = Depends(get_db)):
    db_trade = crud.get_trade(db, trade_id=trade_id)
    if db_trade is None:
        raise HTTPException(status_code=404, detail="Trade not found")
    return crud.update_trade(db, trade_id=trade_id, trade=trade)


@app.delete("/trades/{trade_id}")
def delete_trade(trade_id: int, db: Session = Depends(get_db)):
    db_trade = crud.get_trade(db, trade_id=trade_id)
    if db_trade is None:
        raise HTTPException(status_code=404, detail="Trade not found")
    crud.delete_trade(db, trade_id=trade_id)
    return {"detail": "Trade deleted"}
