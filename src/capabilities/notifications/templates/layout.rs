pub fn add_layout(body: String) -> String {
    format!("<!DOCTYPE html>
<html>

<head>
    <meta name='viewport' content='width=device-width, initial-scale=1.0' />
</head>

<body>
    <table align='center' cellpadding='0' cellspacing='0' border='0' width='600px' style='padding: 0 72px;'>
        <tbody style='font-family: sans-serif'>
            <tr>
                <td style='text-align: center'>
                    <a href='https://www.kodingkorp.com'><img label='Header Image' editable='true'
                            src='https://live-projects.shivammathur.in/kodingkorp/public/logo-bg.png'
                            width='250' border='0' align='top' style='display: inline'></a>
                </td>
            </tr>
        </tbody>
    </table>
    <table align='center' cellpadding='0' cellspacing='0' border='0' width='600px'
        style='padding: 0 72px; margin-bottom: 32px'>
        <tbody style='font-family: sans-serif'>
            {body}
        </tbody>
    </table>
    <table align='center' cellpadding='0' cellspacing='0' border='0' width='600px' style='padding: 0 72px; '>
        <tbody style='font-family: sans-serif'>
            <tr>
                <td
                    style='padding: 24px; background-color: #f5f5f5; color: #7f7f7f; text-align: center; font-size: 14px'>
                    <b>PS:</b> We hope you're enjoying your experience with us! As always, feel free to reach out to us.
                    We'd love to hear from you.
                </td>
            </tr>
        </tbody>
    </table>
</body>

</html>")
}
