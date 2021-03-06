use super::data::Label;
use std::cmp::min;
use std::collections::HashSet;

/// Calculate precision@k metrics.
pub fn precision_at_k<S: ::std::hash::BuildHasher>(
    max_k: usize,
    true_labels: &[HashSet<Label, S>],
    predicted_labels: &[Vec<(Label, f32)>],
) -> Vec<f32> {
    assert_eq!(true_labels.len(), predicted_labels.len());
    let mut ps = vec![0.; max_k];
    for (labels, predictions) in izip!(true_labels, predicted_labels) {
        let mut n_correct = 0;
        for k in 0..min(max_k, predictions.len()) {
            if labels.contains(&predictions[k].0) {
                n_correct += 1;
            }
            ps[k] += n_correct as f32 / (k + 1) as f32;
        }
    }
    for p in &mut ps {
        *p /= true_labels.len() as f32;
    }
    ps
}
