#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""Binance Quote Getter."""

import click as cl
import logging
from datetime import datetime, timedelta
import csv
import requests as req
import yaml


class Binance(object):
    """Binance price fetch object."""

    def __init__(self, config):
        """Init."""
        self.csv = config["csv"]
        self.url = config["url"]
        self.start = config["start"]
        self.end = config["end"]
        self.limit = config.get("limit") or 1000
        self.tfindex = config["timeFieldIndex"]
        self.args = config["urlparams"]
        self.startField = config["timeFieldName"]["start"]
        self.endField = config["timeFieldName"]["end"]
        self.epoch = config.get("epoch", True)
        self.time_unit = config.get("timeUnit") or 1
        self.log = logging.getLogger(__name__)

    def fetch(self):
        """Fetch the data."""
        ret = []
        start_time = self.start or datetime(year=2010, month=1, day=1)
        end_time = self.end or datetime.utcnow()

        while start_time <= end_time:
            next_time = start_time + timedelta(
                days=min((end_time - start_time).days, self.limit)
            )
            args = self.args.copy()
            if self.epoch:
                args[self.startField] = int(
                    start_time.timestamp() * self.time_unit
                )
                args[self.endField] = int(
                    next_time.timestamp() * self.time_unit
                )
            else:
                args[self.startField] = start_time.isoformat()
                args[self.endField] = next_time.isoformat()
            print(
                "Start downloading price data since "
                f"{start_time.date().isoformat()} "
                f"at {next_time.date().isoformat()}."
            )
            resp = req.get(self.url, params=args)
            try:
                resp.raise_for_status()
                price_list = resp.json()
                ret.extend(price_list)
                if price_list:
                    ret = sorted(ret, key=lambda item: item[self.tfindex])
                    start_time = datetime.fromtimestamp(
                        int(ret[-1][self.tfindex]) / self.time_unit
                    )
                else:
                    start_time = next_time
                start_time += timedelta(days=1)
            except req.HTTPError as e:
                self.log.warn(e)
                start_time = next_time
        return [
            item for item in ret
            if datetime.fromtimestamp(
                int(item[self.tfindex]) / self.time_unit
            ).date() < datetime.utcnow().date()
        ]

    def __call__(self):
        """Run the fetcher."""
        self.store(self.csv["out"], self.transform(self.fetch()))

    def transform(self, payload):
        """Transform the data."""
        return [
            dict(zip(self.csv["fields"], item))
            for item in payload
        ]

    def store(self, fname, data):
        """Store the data as csv."""
        with open(fname, 'w') as f:
            w = csv.DictWriter(f, self.csv["fields"])
            w.writeheader()
            w.writerows(data)


@cl.command()
@cl.option(
    "-c", "--config",
    default="config/binance-BTCUSD.yml",
    help="The path of configuration file.",
    type=cl.File()
)
def main(config):
    """Main."""
    cfg_dic = yaml.load(config, Loader=yaml.loader.SafeLoader)
    config.close()
    binance = Binance(cfg_dic)
    binance()
    print("Done.")


if __name__ == "__main__":
    main()
