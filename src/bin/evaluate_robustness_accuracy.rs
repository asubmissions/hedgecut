extern crate hedgecut;

use hedgecut::evaluation::robustness_accuracy;
use hedgecut::dataset::{AdultDataset, CardioDataset, GiveMeSomeCreditDataset, PropublicaDataset, ShoppingDataset};

fn main() {
    let num_trees = 100;
    let min_leaf_size = 2;


    let samples = AdultDataset::samples_from_csv("datasets/adult-train.csv");
    let test_data = AdultDataset::samples_from_csv("datasets/adult-test.csv");
    let dataset = AdultDataset::from_samples(&samples);
    let max_tries_per_split = 5;

    robustness_accuracy(
        "adult",
        dataset,
        samples,
        test_data,
        num_trees,
        min_leaf_size,
        max_tries_per_split
    );


    let samples = CardioDataset::samples_from_csv("datasets/cardio-train.csv");
    let test_data = CardioDataset::samples_from_csv("datasets/cardio-test.csv");
    let dataset = CardioDataset::from_samples(&samples);
    let max_tries_per_split = 5;

    robustness_accuracy(
        "cardio",
        dataset,
        samples,
        test_data,
        num_trees,
        min_leaf_size,
        max_tries_per_split
    );


    let samples = GiveMeSomeCreditDataset::samples_from_csv("datasets/givemesomecredit-train.csv");
    let test_data = GiveMeSomeCreditDataset::samples_from_csv("datasets/givemesomecredit-test.csv");
    let dataset = GiveMeSomeCreditDataset::from_samples(&samples);
    let max_tries_per_split = 5;

    robustness_accuracy(
        "givemesomecredit",
        dataset,
        samples,
        test_data,
        num_trees,
        min_leaf_size,
        max_tries_per_split
    );


    let samples = PropublicaDataset::samples_from_csv("datasets/propublica-train.csv");
    let test_data = PropublicaDataset::samples_from_csv("datasets/propublica-test.csv");
    let dataset = PropublicaDataset::from_samples(&samples);
    let max_tries_per_split = 5;

    robustness_accuracy(
        "propublica",
        dataset,
        samples,
        test_data,
        num_trees,
        min_leaf_size,
        max_tries_per_split
    );


    let samples = ShoppingDataset::samples_from_csv("datasets/shopping-train.csv");
    let test_data = ShoppingDataset::samples_from_csv("datasets/shopping-test.csv");
    let dataset = ShoppingDataset::from_samples(&samples);
    let max_tries_per_split = 50;

    robustness_accuracy(
        "shopping",
        dataset,
        samples,
        test_data,
        num_trees,
        min_leaf_size,
        max_tries_per_split
    );
}