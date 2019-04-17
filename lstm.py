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

    def model(self, input_shape):
        """Return LSTM model."""
        model = tf.keras.models.Sequential([
            tf.keras.layers.CuDNNLSTM(
                units=30, return_sequences=False,
                input_shape=input_shape
            ),
            tf.keras.layers.Dense(
                units=1, activation='linear'
            ),
        ])
        model.compile(optimizer='rmsprop', loss='mean_squared_error')
        return model

    def train(self, X_train, Y_train, X_test, Y_test, out, callbacks=None):
        """Train."""
        model = self.model(input_shape=(None, 1))
        callbacks = callbacks or []
        callbacks.append(tf.keras.callbacks.EarlyStopping(
            monitor='val_loss', mode='auto', patience=0
        ))
        model.fit(
            x=X_train, y=Y_train,
            batch_size=2,
            epochs=15,
            validation_split=0.15,
            callbacks=callbacks
        )
        model.save(out)

    def split_seq(self, size: int):
        """Split the price data into a nested list of the length of size."""
        return [
            self.data[index:index + size].values
            for index in range(len(self.data) - size + 1)
        ]

    def split_data(self, rate: float):
        """
        Split data with data to learn and data to test.

        Parameters:
            rate: the percentage to determine the border of learn-use data and
                test-use data. Must be (0, 1.0)

        """
        if 0.0 >= rate or 1.0 <= rate:
            raise ValueError("The rate must be between >0.1 and <1.0")
        data = np.array(self.split_seq(30))
        row = int(len(data) * rate)
        train = data[:row, :]

        X_train, Y_train = train[:row, :-1], train[:row, -1]
        X_test, Y_test = data[row:, :-1], data[row:, -1]
        X_train, X_test = \
            np.reshape(X_train,  (X_train.shape[0], X_train.shape[1], 1)),\
            np.reshape(X_test,  (X_test.shape[0], X_test.shape[1], 1))
        # Y_train, Y_test = \
        #     np.reshape(Y_train,  (Y_train.shape[0], 1)),\
        #     np.reshape(Y_test,  (Y_test.shape[0], 1))
        return X_train, Y_train, X_test, Y_test


@cl.command()
@cl.argument('fin', type=cl.File())
@cl.argument('field')
@cl.option(
    "-l", "--logdir", type=cl.Path(), default="logs",
    help="Directory to store the log compatible with tensorboard."
)
@cl.option(
    "-o", "--output", type=cl.File(mode='wb'),
    default="lstm_model.hdf",
    help="File path to store the model"
)
def main(output, logdir, field, fin):
    """Main."""
    reader = [dict(item) for item in csv.DictReader(fin)]
    fin.close()
    model = Machine(reader, field)
    (X_train, Y_train, X_test, Y_test) = model.split_data(0.85)
    tb = tf.keras.callbacks.TensorBoard(log_dir=logdir, histogram_freq=1)
    model.train(X_train, Y_train, X_test, Y_test, output, callbacks=[tb])
    output.close()
    print("Done.")


if __name__ == "__main__":
    main()
