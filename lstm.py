#!/usr/bin/env python
# -*- coding: utf-8 -*-

"""Price preditor with LSTM."""

import csv

import numpy as np
import pandas as pd
import sklearn.preprocessing as prep
import tensorflow as tf

import click as cl
import yaml


class Machine(object):
    """Machine."""

    def __init__(self, raw_data):
        """Init."""
        self.raw_data = raw_data

    @property
    def filtered_data(self):
        """Return filtered data by field column."""
        return pd.DataFrame([
            [
                float(item['open']),
                float(item['close']),
                float(item['high']),
                float(item['low']),
            ] for item in self.raw_data
        ])

    @property
    def dates_list(self):
        """Return the recorded date list."""
        return [
            int(item["openTime"]) for item in self.raw_data
        ]

    @property
    def scaler(self):
        """Scaler."""
        data = self.filtered_data
        scaler = prep.MinMaxScaler()
        scaler.fit(data)
        return scaler

    @property
    def data(self):
        """Return preprocessed data."""
        data = data = self.filtered_data
        return pd.DataFrame(self.scaler.transform(data))

    def model(self, input_shape):
        """Return LSTM model."""
        model = tf.keras.models.Sequential([
            tf.keras.layers.CuDNNLSTM(
                units=300, return_sequences=True,
                input_shape=input_shape,
            ),
            tf.keras.layers.Dense(
                units=300, activation='relu'
            ),
            tf.keras.layers.Dropout(0.2),
            tf.keras.layers.CuDNNLSTM(
                units=300, return_sequences=True,
                input_shape=input_shape,
            ),
            tf.keras.layers.Dense(
                units=4, activation='relu',
            ),
            tf.keras.layers.Activation('linear'),
        ])
        model.compile(optimizer='adam', loss='msle')
        return model

    def train(self, X_train, Y_train, X_test, Y_test, out, callbacks=None):
        """Train."""
        model = self.model(input_shape=(None, 4))
        callbacks = callbacks or []
        model.fit(
            x=X_train, y=Y_train,
            epochs=100,
            validation_data=(X_test, Y_test),
            callbacks=callbacks
        )
        model.save(out)

    def split_seq(self, size: int):
        """Split the price data into a nested list of the length of size."""
        data = self.data
        return [
            data[index:index + size].values
            for index in range(len(data) - size + 1)
        ]

    def split_data(
        self, rate: float, num_seq_split: int = 30, dur_pred: int = 1
    ):
        """
        Split data with data to learn and data to test.

        Parameters:
            rate: the percentage to determine the border of learn-use data and
                test-use data. Must be (0, 1.0)
            dur_pred: Duration to predict. Default = 1
            num_seq_split: Number of Sequence to split

        """
        if 0.0 > rate or 1.0 < rate:
            raise ValueError("The rate must be between >0.1 and <1.0")
        if dur_pred >= num_seq_split:
            raise ValueError("The dur_pred must be less than num_seq_split")
        data = np.array(self.split_seq(num_seq_split))
        row = int(len(data) * rate)
        train = data[:row, :]

        X_train, Y_train = train[:row, :-dur_pred], train[:row, dur_pred:]
        X_test, Y_test = data[row:, :-dur_pred], data[row:, dur_pred:]
        X_train, X_test = \
            np.reshape(X_train,  (X_train.shape[0], X_train.shape[1], 4)),\
            np.reshape(X_test,  (X_test.shape[0], X_test.shape[1], 4))
        return X_train, Y_train, X_test, Y_test


@cl.command()
@cl.argument('config', type=cl.File())
def main(config):
    """Main."""
    cfg = yaml.load(config, Loader=yaml.loader.SafeLoader)
    config.close()
    reader = None

    with open(cfg["csv"]["out"]) as fin:
        reader = [dict(item) for item in csv.DictReader(fin)]

    model = Machine(reader)
    (X_train, Y_train, X_test, Y_test) = model.split_data(
        cfg["splitRate"], cfg["seq"], cfg["dur"]
    )
    with open(cfg["model"], "wb") as out:
        model.train(
            X_train, Y_train, X_test, Y_test, out,
            callbacks=[
                tf.keras.callbacks.TensorBoard(
                    log_dir=cfg["logdir"], histogram_freq=1
                )
            ]
        )
    print("Done.")


if __name__ == "__main__":
    main()
