export default function handler(req, res) {
    if (req.method === 'GET') {
        res.status(200).json({ message: "Hello from backend API!" });
    }
}