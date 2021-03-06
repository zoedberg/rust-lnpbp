// LNP/BP Core Library implementing LNPBP specifications & standards
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

#[derive(Clone, PartialEq, Eq, Hash, Debug, Display, Error, From)]
#[display(Debug)]
pub enum Error {
    #[from(zmq::Error)]
    #[from(std::io::Error)]
    SocketError,
    RequiresLocalSocket,
    #[from(!)]
    UnreachableError,
}
