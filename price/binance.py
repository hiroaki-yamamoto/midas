#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""Binance Quote Getter."""

import click as cl
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
        return req.get(self.url, params=self.args).json()

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
