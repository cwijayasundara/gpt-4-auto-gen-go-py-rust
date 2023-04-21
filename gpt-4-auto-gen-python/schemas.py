from pydantic import BaseModel
from datetime import date


class TradeCreate(BaseModel):
    value: float
    date: date
    trader: str


class Trade(TradeCreate):
    id: int

    class Config:
        orm_mode = True
