use fj::core::{
    objects::Solid,
    operations::{
        insert::Insert, split::SplitFace, sweep::SweepFaceOfShell,
        update::UpdateSolid,
    },
};

pub fn model(
    size: f64,
    split_pos: f64,
    core: &mut fj::core::Instance,
) -> Solid {
    let cuboid = cuboid::model([size, size, size], core);

    cuboid.update_shell(cuboid.shells().only(), |shell| {
        let face = shell.faces().first();
        let cycle = face.region().exterior();

        let line = [
            (cycle.half_edges().nth(0).unwrap(), [split_pos]),
            (cycle.half_edges().nth(2).unwrap(), [split_pos]),
        ];

        let (shell, [face, _]) = shell.split_face(face, line, core);

        [shell
            .sweep_face_of_shell(face, [0., 0., -size / 2.], core)
            .insert(&mut core.services)]
    })
}
