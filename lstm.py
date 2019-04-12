#!/usr/bin/env python
# -*- coding: utf-8 -*-

"""Price preditor with LSTM."""

import csv

import numpy as np
import pandas as pd
import sklearn.preprocessing as prep
import tensorflow as tf

import click as cl


class Machine(object):
    """Machine."""

    num_units = 30

    def __init__(self, raw_data, field):
        """Init."""
        self.raw_data = raw_data
        self.field = field

    @property
    def data(self):
        """Return preprocessed data."""
        data = pd.DataFrame([
            float(item[self.field]) for item in self.raw_data
        ])
        scaler = prep.MinMaxScaler()
        return pd.DataFrame(scaler.fit_transform(data))

    @property
    def model(self):
        """Return LSTM model."""
        model = tf.keras.models.Sequential([
            tf.keras.layers.LSTM(
                unis=self.num_units, return_sequences=True,
                input_shape=(None, 1)
            ),
            tf.keras.layers.Dense(
                unis=1, activation='linear'
            ),
        ])
        model.compile(optimizer='rmsprop', loss='mean_squared_error')
        return model

    def train(self, X_train, Y_train, out):
        """Train."""
        model = self.model
        model.fit(
            x=X_train, y=Y_train,
            batch_size=2,
            epochs=15,
            validation_split=0.05
        )
        model.save(out)

    def split_data(self, rate: float):
        """
        Split data with data to learn and data to test.

        Parameters:
            rate: the percentage to determine the border of learn-use data and
                test-use data. Must be (0, 1.0)

        """
        if 0.0 >= rate or 1.0 <= rate:
            raise ValueError("The rate must be between >0.1 and <1.0")
        data = np.array(self.data)
        row = int(len(data) * rate)
        train = data[:row, :]

        X_train, Y_train = train[:row, :-1], train[:row, -1]
        X_test, Y_test = data[row:, :-1], data[row:, -1]
        X_train, X_test = \
            np.reshape(X_train,  (X_train.shape[0], X_train.shape[1], 1)),\
            np.reshape(X_test,  (X_test.shape[0], X_test.shape[1], 1))
        return X_train, Y_train, X_test, Y_test


@cl.command()
@cl.argument('fin', type=cl.File())
@cl.argument('field')
@cl.option(
    "-o", "--output", type=cl.File(mode='wb'),
    default="sltm_model.hdf",
    help="File path to store the model"
)
def main(output, field, fin):
    """Main."""
    reader = [dict(item) for item in csv.DictReader(fin)]
    fin.close()
    model = Machine(reader, field)
    (X_train, Y_train, X_test, Y_test) = model.split_data(0.85)
    model.train(X_train, Y_train, output)
    output.close()
    print("Done.")


if __name__ == "__main__":
    main()
