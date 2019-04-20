#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""Binance Quote Getter."""

import click as cl
from datetime import datetime, timedelta
import csv
import requests as req


class Binance(object):
    """Binance price fetch object."""

    url = "https://api.binance.com/api/v1/klines"
    args = {
        "symbol": "BTCUSDT",
        "interval": "1d",
        "limit": 1000,
    }
    fields = [
        "openTime", "open", "high", "low", "close",
        "volume", "closeTime", "asset_vol", "num_trades"
    ]

    def fetch(self):
        """Fetch the data."""
        ret = []
        start_time = datetime(year=2010, month=1, day=1)
        while start_time <= datetime.utcnow():
            end_time = start_time + timedelta(days=1000)
            args = self.args.copy()
            args["startTime"] = int(start_time.timestamp() * 1000)
            args["endTime"] = int(end_time.timestamp() * 1000)
            print(
                "Start downloading price data since "
                f"{start_time.date().isoformat()} "
                f"at {end_time.date().isoformat()}."
            )
            resp = req.get(self.url, params=self.args)
            resp.raise_for_status()
            ret.extend(resp.json())
            start_time = end_time + timedelta(days=1)

        return [
            item for item in ret
            if datetime.utcfromtimestamp(
                item[0] / 1000
            ).date() < datetime.utcnow().date()
        ]

    def run(self, filename):
        """Run the fetcher."""
        self.store(filename, self.transform(self.fetch()))

    def transform(self, payload):
        """Transform the data."""
        return [
            dict(zip(self.fields, item))
            for item in payload
        ]

    def store(self, fname, data):
        """Store the data as csv."""
        with open(fname, 'w') as f:
            w = csv.DictWriter(f, self.fields)
            w.writeheader()
            w.writerows(data)


@cl.command()
@cl.option(
    "-o", "--output",
    default="binance-BTCUSD.csv",
    help="File name to output",
    type=cl.Path()
)
def main(output):
    """Main."""
    binance = Binance()
    binance.run(output)
    print("Done.")


if __name__ == "__main__":
    main()
