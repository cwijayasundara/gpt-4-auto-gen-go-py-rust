from sqlalchemy import Column, Integer, Float, Date, String
from database import Base


class Trade(Base):
    __tablename__ = "trades"

    id = Column(Integer, primary_key=True, index=True)
    value = Column(Float, nullable=False)
    date = Column(Date, nullable=False)
    trader = Column(String, nullable=False)
