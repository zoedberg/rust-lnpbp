// LNP/BP Rust Library
// Written in 2020 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

use std::collections::BTreeSet;

use bitcoin::Txid;

use crate::bp;
use crate::bp::blind::OutpointReveal;
use crate::rgb::{
    validation, Anchor, Extension, Genesis, Node, NodeId, Schema, Transition,
    Validator,
};

pub type ConsignmentEndpoints = Vec<(NodeId, bp::blind::OutpointHash)>;
pub type TransitionData = Vec<(Anchor, Transition)>;
pub type ExtensionData = Vec<Extension>;

pub const RGB_CONSIGNMENT_VERSION: u16 = 0;

#[derive(Clone, Debug, Display, StrictEncode, StrictDecode)]
#[strict_crate(crate)]
#[display(Debug)]
pub struct Consignment {
    version: u16,
    pub genesis: Genesis,
    pub endpoints: ConsignmentEndpoints,
    pub state_transitions: TransitionData,
    pub state_extensions: ExtensionData,
}

impl Consignment {
    pub fn with(
        genesis: Genesis,
        endpoints: ConsignmentEndpoints,
        state_transitions: TransitionData,
        state_extensions: ExtensionData,
    ) -> Consignment {
        Self {
            version: RGB_CONSIGNMENT_VERSION,
            genesis,
            endpoints,
            state_extensions,
            state_transitions,
        }
    }

    #[inline]
    pub fn txids(&self) -> BTreeSet<Txid> {
        self.state_transitions
            .iter()
            .map(|(anchor, _)| anchor.txid)
            .collect()
    }

    #[inline]
    pub fn node_ids(&self) -> BTreeSet<NodeId> {
        let mut set = bset![self.genesis.node_id()];
        set.extend(
            self.state_transitions
                .iter()
                .map(|(_, node)| node.node_id()),
        );
        set.extend(self.state_extensions.iter().map(Extension::node_id));
        set
    }

    pub fn validate<R: validation::TxResolver>(
        &self,
        schema: &Schema,
        resolver: R,
    ) -> validation::Status {
        Validator::validate(schema, self, resolver)
    }

    /// Reveals previously known seal information (replacing blind UTXOs with
    /// unblind ones). Function is used when a peer receives consignment
    /// containing concealed seals for the outputs owned by the peer
    pub fn reveal_seals<'a>(
        &mut self,
        known_seals: impl Iterator<Item = &'a OutpointReveal> + Clone,
    ) -> usize {
        let counter = 0;
        for (_, transition) in &mut self.state_transitions {
            transition.owned_rights_mut().into_iter().fold(
                counter,
                |counter, (_, assignment)| {
                    counter + assignment.reveal_seals(known_seals.clone())
                },
            );
        }
        for extension in &mut self.state_extensions {
            extension.owned_rights_mut().into_iter().fold(
                counter,
                |counter, (_, assignment)| {
                    counter + assignment.reveal_seals(known_seals.clone())
                },
            );
        }
        counter
    }
}

#[cfg(test)]
pub(crate) mod test {
    use super::*;
    use crate::rgb::schema::test::schema;
    use crate::rgb::validation::TxResolver;
    use crate::strict_encoding::StrictDecode;

    pub(crate) fn consignment() -> Consignment {
        let data: Vec<u8> = vec![
            0, 0, 81, 189, 152, 202, 208, 73, 28, 146, 80, 53, 236, 43, 105,
            76, 225, 179, 161, 203, 253, 49, 8, 205, 244, 160, 80, 216, 216,
            251, 188, 226, 52, 13, 1, 0, 153, 0, 67, 73, 127, 215, 248, 38,
            149, 113, 8, 244, 163, 15, 217, 206, 195, 174, 186, 121, 151, 32,
            132, 233, 14, 173, 1, 234, 51, 9, 0, 0, 0, 0, 7, 0, 116, 101, 115,
            116, 110, 101, 116, 11, 17, 9, 7, 4, 0, 116, 101, 115, 116, 2, 0,
            116, 98, 157, 71, 156, 71, 1, 0, 0, 0, 236, 1, 28, 0, 0, 34, 2, 0,
            0, 0, 0, 0, 0, 4, 0, 116, 66, 84, 67, 12, 0, 84, 101, 115, 116, 32,
            66, 105, 116, 99, 111, 105, 110, 12, 0, 84, 101, 115, 116, 32, 115,
            97, 116, 111, 115, 104, 105, 0, 225, 245, 5, 0, 0, 0, 0, 67, 73,
            127, 215, 248, 38, 149, 113, 8, 244, 163, 15, 217, 206, 195, 174,
            186, 121, 151, 32, 132, 233, 14, 173, 1, 234, 51, 9, 0, 0, 0, 0, 0,
            1, 1, 7, 0, 0, 0, 1, 0, 33, 4, 0, 85, 83, 68, 84, 1, 0, 1, 0, 33,
            10, 0, 85, 83, 68, 32, 84, 101, 116, 104, 101, 114, 3, 0, 1, 0, 3,
            160, 134, 1, 0, 0, 0, 0, 0, 4, 0, 1, 0, 3, 160, 134, 1, 0, 0, 0, 0,
            0, 5, 0, 1, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 1, 0, 0, 0, 8, 0,
            1, 0, 11, 98, 7, 127, 95, 0, 0, 0, 0, 2, 0, 1, 0, 1, 3, 1, 0, 1, 0,
            178, 33, 37, 44, 27, 178, 4, 3, 40, 73, 173, 36, 33, 53, 221, 119,
            251, 0, 189, 217, 213, 41, 198, 175, 58, 121, 140, 28, 146, 37, 87,
            64, 38, 143, 99, 202, 152, 114, 142, 39, 0, 0, 0, 0, 3, 160, 134,
            1, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 0, 0,
            0, 0, 0, 0, 0, 1, 0, 92, 166, 163, 138, 199, 202, 164, 245, 113,
            122, 18, 103, 23, 163, 19, 162, 163, 76, 87, 249, 108, 157, 128,
            35, 167, 241, 108, 90, 119, 62, 221, 25, 13, 89, 171, 84, 193, 91,
            149, 210, 253, 90, 73, 209, 95, 67, 176, 176, 241, 245, 106, 115,
            168, 19, 65, 93, 243, 55, 106, 242, 165, 8, 73, 84, 1, 0, 168, 170,
            15, 228, 35, 104, 92, 41, 63, 185, 164, 121, 156, 162, 131, 12, 47,
            6, 164, 26, 196, 153, 222, 217, 64, 160, 41, 209, 233, 153, 122,
            12, 3, 0, 1, 99, 9, 176, 91, 232, 64, 240, 92, 218, 231, 109, 250,
            143, 250, 176, 91, 23, 191, 14, 131, 20, 25, 114, 222, 246, 150,
            230, 15, 107, 177, 120, 169, 92, 166, 163, 138, 199, 202, 164, 245,
            113, 122, 18, 103, 23, 163, 19, 162, 163, 76, 87, 249, 108, 157,
            128, 35, 167, 241, 108, 90, 119, 62, 221, 25, 0, 195, 21, 162, 3,
            239, 233, 201, 237, 18, 236, 101, 93, 131, 155, 149, 138, 74, 215,
            139, 198, 25, 200, 138, 206, 173, 165, 44, 179, 145, 229, 87, 33,
            0, 100, 123, 211, 136, 254, 136, 113, 24, 177, 157, 79, 243, 71,
            248, 33, 143, 160, 201, 21, 73, 103, 215, 72, 106, 243, 186, 141,
            61, 139, 200, 9, 68, 1, 69, 24, 189, 99, 171, 32, 233, 39, 3, 158,
            255, 31, 84, 122, 29, 95, 146, 223, 162, 186, 122, 246, 172, 151,
            26, 75, 208, 59, 164, 167, 52, 176, 49, 86, 162, 86, 184, 173, 58,
            30, 249, 0, 1, 0, 0, 0, 1, 0, 99, 9, 176, 91, 232, 64, 240, 92,
            218, 231, 109, 250, 143, 250, 176, 91, 23, 191, 14, 131, 20, 25,
            114, 222, 246, 150, 230, 15, 107, 177, 120, 169, 1, 0, 1, 0, 1, 0,
            0, 0, 1, 0, 1, 0, 1, 3, 2, 0, 2, 13, 89, 171, 84, 193, 91, 149,
            210, 253, 90, 73, 209, 95, 67, 176, 176, 241, 245, 106, 115, 168,
            19, 65, 93, 243, 55, 106, 242, 165, 8, 73, 84, 3, 100, 0, 0, 0, 0,
            0, 0, 0, 32, 0, 163, 255, 181, 52, 214, 207, 164, 188, 164, 62,
            206, 51, 148, 21, 55, 20, 219, 87, 167, 93, 37, 93, 220, 174, 85,
            114, 39, 10, 129, 149, 142, 205, 3, 0, 29, 168, 83, 31, 208, 67,
            243, 121, 190, 173, 32, 142, 81, 240, 183, 249, 253, 225, 241, 208,
            43, 50, 220, 113, 160, 83, 117, 66, 39, 54, 213, 157, 241, 15, 215,
            23, 180, 230, 84, 244, 1, 0, 0, 0, 33, 0, 8, 51, 208, 63, 29, 66,
            223, 18, 233, 176, 231, 97, 133, 123, 94, 138, 167, 67, 249, 121,
            198, 119, 104, 110, 244, 210, 160, 32, 50, 229, 234, 213, 247, 163,
            2, 80, 167, 74, 173, 71, 114, 240, 90, 198, 161, 11, 196, 138, 22,
            251, 17, 118, 223, 43, 233, 180, 69, 93, 26, 50, 211, 38, 66, 218,
            207, 135, 95, 102, 240, 157, 211, 197, 152, 172, 141, 203, 255, 91,
            167, 227, 63, 121, 238, 202, 205, 162, 113, 232, 26, 200, 160, 249,
            72, 35, 41, 84, 162, 219, 234, 14, 171, 22, 87, 37, 133, 199, 33,
            100, 238, 228, 194, 63, 189, 223, 251, 234, 251, 72, 86, 205, 194,
            119, 144, 187, 48, 167, 189, 142, 6, 97, 181, 165, 165, 12, 93, 80,
            225, 87, 26, 182, 106, 177, 201, 169, 122, 10, 106, 238, 67, 209,
            103, 180, 129, 223, 15, 76, 127, 129, 105, 153, 37, 192, 128, 201,
            59, 239, 226, 124, 211, 198, 255, 199, 89, 250, 82, 179, 199, 170,
            37, 79, 28, 18, 171, 37, 124, 213, 153, 43, 1, 20, 30, 121, 123,
            20, 46, 158, 187, 184, 86, 155, 230, 49, 131, 65, 142, 176, 138,
            98, 135, 225, 192, 164, 112, 117, 246, 11, 56, 178, 183, 48, 30,
            189, 160, 12, 240, 160, 58, 220, 31, 104, 93, 17, 116, 167, 162,
            10, 115, 38, 90, 118, 209, 57, 200, 87, 204, 18, 113, 139, 33, 34,
            58, 41, 240, 44, 240, 17, 168, 2, 101, 139, 176, 220, 59, 52, 163,
            116, 179, 237, 35, 167, 202, 9, 53, 168, 12, 206, 239, 238, 37, 36,
            226, 148, 200, 150, 67, 44, 14, 84, 248, 155, 229, 229, 131, 137,
            171, 156, 87, 55, 136, 196, 240, 114, 166, 202, 79, 60, 107, 113,
            3, 120, 145, 10, 238, 210, 216, 197, 23, 4, 48, 225, 74, 80, 249,
            204, 211, 175, 197, 14, 90, 222, 58, 206, 208, 70, 249, 223, 34,
            112, 175, 246, 53, 53, 29, 40, 155, 183, 193, 19, 178, 160, 162,
            195, 197, 108, 35, 141, 93, 79, 32, 83, 76, 37, 139, 53, 177, 155,
            33, 129, 32, 231, 37, 207, 243, 223, 172, 72, 230, 20, 233, 121,
            175, 182, 186, 103, 49, 232, 49, 124, 7, 1, 246, 154, 70, 205, 72,
            137, 138, 13, 9, 112, 100, 34, 95, 14, 125, 184, 196, 241, 191, 85,
            83, 199, 84, 121, 52, 180, 123, 223, 99, 254, 82, 11, 209, 165,
            211, 143, 44, 86, 12, 84, 103, 13, 89, 167, 164, 107, 214, 46, 139,
            143, 252, 204, 167, 220, 134, 249, 27, 180, 82, 116, 111, 184, 235,
            127, 170, 181, 197, 22, 182, 72, 52, 4, 88, 155, 204, 242, 213, 31,
            128, 151, 53, 148, 202, 141, 251, 23, 45, 245, 232, 197, 81, 48,
            67, 206, 64, 58, 130, 168, 228, 244, 117, 115, 20, 61, 219, 148,
            200, 253, 44, 19, 168, 238, 15, 165, 205, 97, 243, 0, 185, 50, 231,
            27, 81, 174, 252, 232, 12, 237, 89, 56, 190, 63, 67, 87, 64, 79,
            136, 214, 182, 119, 105, 227, 51, 63, 93, 64, 9, 79, 38, 96, 247,
            84, 176, 176, 205, 28, 239, 101, 12, 245, 113, 200, 102, 0, 158,
            153, 222, 164, 243, 143, 205, 125, 192, 88, 99, 172, 46, 233, 84,
            139, 227, 179, 4, 98, 135, 182, 41, 225, 63, 161, 216, 188, 189,
            169, 38, 98, 193, 22, 128, 190, 36, 211, 1, 199, 30, 55, 185, 172,
            149, 136, 82, 218, 124, 47, 98, 206, 135, 55, 211, 66, 12, 158,
            202, 114, 153, 151, 113, 182, 6, 55, 34, 174, 166, 119, 187, 144,
            87, 181, 137, 241, 30, 183, 117, 124, 141, 86, 215, 241, 183, 101,
            87, 94, 25, 71, 200, 2, 17, 46, 42, 57, 87, 227, 183, 155, 118,
            136, 95, 162, 169, 102, 197, 8, 138, 61, 128, 231, 202, 17, 103,
            238, 8, 58, 17, 255, 77, 115, 33, 240, 230, 50, 73, 229, 170, 127,
            242, 192, 210, 192, 239, 205, 87, 228, 71, 214, 21, 120, 80, 143,
            125, 203, 213, 131, 78, 231, 61, 117, 55, 145, 98, 144, 0, 0, 0, 0,
            0, 0,
        ];

        Consignment::strict_decode(&data[..]).unwrap()
    }

    struct TestResolver;

    impl TxResolver for TestResolver {
        fn resolve(
            &self,
            txid: &Txid,
        ) -> Result<
            Option<(bitcoin::Transaction, u64)>,
            validation::TxResolverError,
        > {
            eprintln!("Validating txid {}", txid);
            Err(validation::TxResolverError)
        }
    }

    #[test]
    fn test_consignment_validation() {
        let consignment = consignment();
        let schema = schema();
        let status = consignment.validate(&schema, TestResolver);
        println!("{}", status);
    }
}
