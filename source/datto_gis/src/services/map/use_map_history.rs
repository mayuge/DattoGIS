enum ChangeCategory {
    Created{
        id: String,
        after:Arc<Feature>,
    },
    Deleted{
        id: String,
        before:Arc<Feature>,
    },
    Updated{
        id: String,
        before:Arc<Feature>,
        after:Arc<Feature>,
    },
}

struct MapHistory {
    changes: Vec<ChangeCategory>,
}