from pathlib import Path

import numpy as np
from tensorflow.keras.models import Model

from ml4a.nicer_phase_amplitudes_to_parameters_models import Mira
from ml4a.paths import user_data_directory


def load_trained_phase_amplitudes_to_parameters_model() -> Model:
    model = Mira()
    model.load_weights(
        user_data_directory.joinpath('model_states/infer_parameters_from_phase_amplitudes_model_state/model.ckpt'))
    return model

def split_array_into_chunks(array: np.ndarray, chunk_size: int):
    return np.split(array, np.arange(chunk_size, len(array), chunk_size))
